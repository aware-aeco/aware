param([Parameter(Mandatory=$true)][string]$RequestPath)
$ErrorActionPreference = 'Stop'
$request = Get-Content -LiteralPath $RequestPath -Raw -Encoding UTF8 | ConvertFrom-Json
if ([IntPtr]::Size -ne 8) { throw 'Compiler auditor requires Windows x64' }
# CodeDOM's legacy compiler cannot use all Unicode temporary paths. This is
# solely the authenticated bootstrap's scratch area; compiler environments below
# still come from the declared request, including their original TEMP/TMP paths.
$auditTempParent = [IO.Path]::GetFullPath([IO.Path]::Combine($env:SystemRoot, 'Temp'))
if ($env:TEMP -ne $auditTempParent -or $env:TMP -ne $auditTempParent -or $auditTempParent -match '[^\x20-\x7e]') { throw 'Invalid auditor temporary parent' }
if (([IO.File]::GetAttributes($auditTempParent) -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw 'Auditor temporary parent redirects elsewhere' }
$auditOwner = [Security.Principal.WindowsIdentity]::GetCurrent().User
$auditAcl = [Security.AccessControl.DirectorySecurity]::new()
$auditAcl.SetAccessRuleProtection($true, $false)
$auditAcl.SetOwner($auditOwner)
foreach ($identity in @($auditOwner, [Security.Principal.SecurityIdentifier]::new('S-1-5-18'))) {
    $auditAcl.AddAccessRule([Security.AccessControl.FileSystemAccessRule]::new($identity, 'FullControl', 'ContainerInherit, ObjectInherit', 'None', 'Allow'))
}
$auditTempPath = [IO.Path]::Combine($auditTempParent, 'aware-compiler-audit-' + [Guid]::NewGuid().ToString('N'))
if ([IO.Directory]::Exists($auditTempPath) -or [IO.File]::Exists($auditTempPath)) { throw 'Auditor temporary path already exists' }
$auditDirectory = [IO.Directory]::CreateDirectory($auditTempPath, $auditAcl)
try {
    if (($auditDirectory.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw 'Auditor temporary directory redirects elsewhere' }
    $actualAcl = $auditDirectory.GetAccessControl()
    if (!$actualAcl.AreAccessRulesProtected -or $actualAcl.GetSecurityDescriptorSddlForm('Access, Owner') -ne $auditAcl.GetSecurityDescriptorSddlForm('Access, Owner')) { throw 'Auditor temporary access rules differ' }
    $env:TEMP = $auditTempPath
    $env:TMP = $auditTempPath
Add-Type -TypeDefinition @'
using System;
using System.IO;
using System.Text;
using System.Diagnostics;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Security.Cryptography;
using Microsoft.Win32.SafeHandles;

public static class AwareCompilerAudit {
  [StructLayout(LayoutKind.Sequential, CharSet=CharSet.Unicode)] struct Startup {
    public uint cb; public string reserved, desktop, title;
    public uint x,y,xSize,ySize,xChars,yChars,fill,flags; public ushort show, reservedSize;
    public IntPtr reservedBytes, input, output, error;
  }
  [StructLayout(LayoutKind.Sequential)] struct ProcessInfo { public IntPtr process, thread; public uint pid, tid; }
  [StructLayout(LayoutKind.Sequential)] struct Accounting {
    public long user,kernel,periodUser,periodKernel; public uint faults,total,active,terminated;
  }
  public class Image { public uint pid; public string path,kind,sha256; public long size; }
  public class ProcessRecord { public uint pid; public string path,action="observed"; public uint? exitCode; }
  public class DeniedImage { public string closure,relativePath,path,sha256; public long size; public uint exitCode; }
  public class StartupPolicy { public string identity; public DeniedImage deniedImage; }
  public class Report {
    public string schema="aware-compiler-debug-audit/v2", error;
    public StartupPolicy startupPolicy;
    public bool complete; public uint exitCode=uint.MaxValue,totalProcesses,activeProcesses;
    public List<ProcessRecord> processes=new List<ProcessRecord>();
    public List<Image> images=new List<Image>();
  }
  [DllImport("kernel32.dll",CharSet=CharSet.Unicode)] static extern uint GetSystemWindowsDirectoryW(StringBuilder text,uint size);
  [DllImport("kernel32.dll",CharSet=CharSet.Unicode)] static extern uint GetSystemDirectoryW(StringBuilder text,uint size);
  [DllImport("kernel32.dll",SetLastError=true,CharSet=CharSet.Unicode)] static extern bool CreateProcessW(string app,StringBuilder command,IntPtr pa,IntPtr ta,bool inherit,uint flags,IntPtr environment,string cwd,ref Startup startup,out ProcessInfo process);
  [DllImport("kernel32.dll",SetLastError=true)] static extern bool WaitForDebugEventEx(IntPtr debugEvent,uint milliseconds);
  [DllImport("kernel32.dll",SetLastError=true)] static extern bool ContinueDebugEvent(uint pid,uint tid,uint status);
  [DllImport("kernel32.dll",SetLastError=true)] static extern bool DebugSetProcessKillOnExit(bool kill);
  [DllImport("kernel32.dll",SetLastError=true,CharSet=CharSet.Unicode)] static extern uint GetFinalPathNameByHandleW(IntPtr file,StringBuilder path,uint size,uint flags);
  [DllImport("kernel32.dll",SetLastError=true,CharSet=CharSet.Unicode)] static extern bool QueryFullProcessImageNameW(IntPtr process,uint flags,StringBuilder path,ref uint size);
  [DllImport("psapi.dll",SetLastError=true,CharSet=CharSet.Unicode)] static extern uint GetMappedFileNameW(IntPtr process,IntPtr address,StringBuilder path,uint size);
  [DllImport("kernel32.dll",SetLastError=true,CharSet=CharSet.Unicode)] static extern uint QueryDosDeviceW(string device,StringBuilder target,uint size);
  [DllImport("kernel32.dll",SetLastError=true,CharSet=CharSet.Unicode)] static extern IntPtr CreateJobObjectW(IntPtr attributes,string name);
  [DllImport("kernel32.dll",SetLastError=true)] static extern bool SetInformationJobObject(IntPtr job,int kind,IntPtr information,uint size);
  [DllImport("kernel32.dll",SetLastError=true)] static extern bool AssignProcessToJobObject(IntPtr job,IntPtr process);
  [DllImport("kernel32.dll",SetLastError=true)] static extern bool QueryInformationJobObject(IntPtr job,int kind,out Accounting info,uint size,IntPtr returned);
  [DllImport("kernel32.dll",SetLastError=true)] static extern bool TerminateJobObject(IntPtr job,uint code);
  [DllImport("kernel32.dll",SetLastError=true)] static extern bool TerminateProcess(IntPtr process,uint code);
  [DllImport("kernel32.dll")] static extern IntPtr GetStdHandle(int id);
  [DllImport("kernel32.dll")] static extern IntPtr GetCurrentProcess();
  [DllImport("kernel32.dll",SetLastError=true)] static extern bool DuplicateHandle(IntPtr sourceProcess,IntPtr source,IntPtr targetProcess,out IntPtr target,uint access,bool inherit,uint options);
  [DllImport("kernel32.dll",SetLastError=true)] static extern bool CloseHandle(IntPtr handle);
  static void Check(bool ok,string operation) { if(!ok) throw new System.ComponentModel.Win32Exception(Marshal.GetLastWin32Error(),operation); }
  public static string[] Host() {
    var windows=new StringBuilder(32768); var system=new StringBuilder(32768);
    uint a=GetSystemWindowsDirectoryW(windows,32768),b=GetSystemDirectoryW(system,32768);
    if(a==0||a>=32768||b==0||b>=32768) throw new Exception("Windows directory API failed");
    return new string[]{Path.GetFullPath(windows.ToString()),Path.GetFullPath(system.ToString())};
  }
  static string DosPath(string path) {
    if(path.StartsWith(@"\\?\",StringComparison.Ordinal)) path=path.Substring(4);
    if(path.StartsWith(@"\Device\",StringComparison.OrdinalIgnoreCase)) {
      foreach(string drive in Environment.GetLogicalDrives()) {
        var target=new StringBuilder(32768);
        if(QueryDosDeviceW(drive.Substring(0,2),target,32768)>0) {
          string prefix=target.ToString();
          if(path.StartsWith(prefix+"\\",StringComparison.OrdinalIgnoreCase)) return Path.GetFullPath(drive.Substring(0,2)+path.Substring(prefix.Length));
        }
      }
      throw new Exception("Unresolved mapped-file device path");
    }
    if(path.Length<3||path[1]!=':'||path[2]!='\\') throw new Exception("Image lacks a local canonical path");
    return Path.GetFullPath(path);
  }
  static Image Capture(uint pid,string kind,IntPtr file,IntPtr process,IntPtr address) {
    var text=new StringBuilder(32768); uint length=0;
    if(file!=IntPtr.Zero&&file!=new IntPtr(-1)) length=GetFinalPathNameByHandleW(file,text,32768,0);
    if(length==0) {
      text.Clear();
      if(kind=="process") { length=32768; Check(QueryFullProcessImageNameW(process,0,text,ref length),"QueryFullProcessImageName"); }
      else length=GetMappedFileNameW(process,address,text,32768);
    }
    if(length==0||length>=32768) throw new Exception("Unresolved compiler image provenance");
    string path=DosPath(text.ToString());
    using(var stream=file!=IntPtr.Zero&&file!=new IntPtr(-1)
      ? new FileStream(new SafeFileHandle(file,false),FileAccess.Read)
      : new FileStream(path,FileMode.Open,FileAccess.Read,FileShare.ReadWrite|FileShare.Delete))
    using(var sha=SHA256.Create()) {
      stream.Position=0;
      return new Image {pid=pid,kind=kind,path=path,size=stream.Length,sha256=BitConverter.ToString(sha.ComputeHash(stream)).Replace("-","").ToLowerInvariant()};
    }
  }
  public static string Quote(string value) {
    var result=new StringBuilder("\""); int slashes=0;
    foreach(char c in value) {
      if(c=='\\') {slashes++;continue;}
      if(c=='\"') {result.Append('\\',slashes*2+1);result.Append(c);slashes=0;continue;}
      result.Append('\\',slashes);slashes=0;result.Append(c);
    }
    result.Append('\\',slashes*2);result.Append('"');return result.ToString();
  }
  static IntPtr Inherit(int id) {
    IntPtr result; var current=GetCurrentProcess();
    Check(DuplicateHandle(current,GetStdHandle(id),current,out result,0,true,2),"Duplicate standard handle"); return result;
  }
  public static Report Run(string executable,string[] args,string cwd,string environment,int timeoutMs,StartupPolicy policy) {
    var report=new Report(); var handles=new Dictionary<uint,IntPtr>(); var records=new Dictionary<uint,ProcessRecord>();
    var breakpoints=new HashSet<uint>(); var active=new HashSet<uint>();
    IntPtr job=IntPtr.Zero,limits=IntPtr.Zero,eventBuffer=IntPtr.Zero,env=IntPtr.Zero;
    IntPtr input=IntPtr.Zero,output=IntPtr.Zero,error=IntPtr.Zero; ProcessInfo initial=new ProcessInfo();
    bool assigned=false; var clock=Stopwatch.StartNew();
    try {
      if(IntPtr.Size!=8) throw new Exception("Debugger requires Windows x64");
      if(policy==null||policy.identity!="aware-private-msvc-telemetry-denial/v1") throw new Exception("Unknown compiler startup policy");
      DeniedImage denied=policy.deniedImage;
      if(denied!=null) {
        if(denied.closure!="compiler-msvc-bin"||!String.Equals(denied.relativePath,"vctip.exe",StringComparison.OrdinalIgnoreCase)
          ||denied.exitCode!=0xe0000488u||denied.size<=0||!System.Text.RegularExpressions.Regex.IsMatch(denied.sha256??"","^[0-9a-f]{64}$")
          ||!String.Equals(Path.GetFileName(denied.path),"vctip.exe",StringComparison.OrdinalIgnoreCase)
          ||DosPath(denied.path)!=denied.path) throw new Exception("Invalid private telemetry policy");
        if(String.Equals(DosPath(executable),denied.path,StringComparison.OrdinalIgnoreCase)) throw new Exception("Root cannot be denied telemetry");
      }
      // These are the effective values used below, including an explicit null if absent.
      report.startupPolicy=policy;
      job=CreateJobObjectW(IntPtr.Zero,null);Check(job!=IntPtr.Zero,"Create owned job");
      // x64 JOBOBJECT_EXTENDED_LIMIT_INFORMATION: basic flags at offset 16; total size 144.
      limits=Marshal.AllocHGlobal(144); for(int i=0;i<144;i++)Marshal.WriteByte(limits,i,0);
      Marshal.WriteInt32(limits,16,0x2000); // KILL_ON_JOB_CLOSE; no breakaway flags.
      Check(SetInformationJobObject(job,9,limits,144),"Set owned job limits");
      input=Inherit(-10);output=Inherit(-11);error=Inherit(-12);
      var startup=new Startup {cb=(uint)Marshal.SizeOf(typeof(Startup)),flags=0x100,input=input,output=output,error=error};
      var command=new StringBuilder(Quote(executable)); foreach(string arg in args)command.Append(" ").Append(Quote(arg));
      env=Marshal.StringToHGlobalUni(environment); eventBuffer=Marshal.AllocHGlobal(176);
      Check(CreateProcessW(executable,command,IntPtr.Zero,IntPtr.Zero,true,0x08000401,env,cwd,ref startup,out initial),"Create private debug process");
      Check(AssignProcessToJobObject(job,initial.process),"Assign private debug process to job");assigned=true;
      Check(DebugSetProcessKillOnExit(true),"Keep private debugger kill-on-exit");
      bool rootExited=false;
      while(true) {
        if(clock.ElapsedMilliseconds>timeoutMs)throw new Exception("Compiler audit deadline exceeded");
        if(!WaitForDebugEventEx(eventBuffer,100)) {
          int code=Marshal.GetLastWin32Error(); if(code!=121)throw new System.ComponentModel.Win32Exception(code,"WaitForDebugEventEx");
          if(rootExited&&active.Count==0)break; continue;
        }
        uint kind=(uint)Marshal.ReadInt32(eventBuffer,0),pid=(uint)Marshal.ReadInt32(eventBuffer,4),tid=(uint)Marshal.ReadInt32(eventBuffer,8);
        uint status=0x00010002; // DBG_CONTINUE for nonexception events.
        if(kind==3) {
          IntPtr file=Marshal.ReadIntPtr(eventBuffer,16),process=Marshal.ReadIntPtr(eventBuffer,24);
          try {
            Image image=Capture(pid,"process",file,process,Marshal.ReadIntPtr(eventBuffer,40));report.images.Add(image);
            var record=new ProcessRecord {pid=pid,path=image.path};report.processes.Add(record);records.Add(pid,record);handles.Add(pid,process);active.Add(pid);
            if(denied!=null&&String.Equals(image.path,denied.path,StringComparison.OrdinalIgnoreCase)) {
              if(pid==initial.pid||image.size!=denied.size||image.sha256!=denied.sha256) throw new Exception("Telemetry creation differs from private authority");
              // The creation event has not been continued: no user entry point has run.
              Check(TerminateProcess(process,denied.exitCode),"Deny private telemetry startup");
              record.action="blocked-telemetry";
            }
          } finally {if(file!=IntPtr.Zero&&file!=new IntPtr(-1))CloseHandle(file);}
        } else if(kind==6) {
          IntPtr file=Marshal.ReadIntPtr(eventBuffer,16);
          try {if(!handles.ContainsKey(pid))throw new Exception("DLL event has no observed process");report.images.Add(Capture(pid,"dll",file,handles[pid],Marshal.ReadIntPtr(eventBuffer,24)));}
          finally {if(file!=IntPtr.Zero&&file!=new IntPtr(-1))CloseHandle(file);}
        } else if(kind==5) {
          if(!records.ContainsKey(pid))throw new Exception("Exit event has no observed process");
          uint exit=(uint)Marshal.ReadInt32(eventBuffer,16);records[pid].exitCode=exit;active.Remove(pid);handles.Remove(pid);
          if(pid==initial.pid){rootExited=true;report.exitCode=exit;}
        } else if(kind==1) {
          uint exception=(uint)Marshal.ReadInt32(eventBuffer,16);
          status=exception==0x80000003 && breakpoints.Add(pid)?0x00010002u:0x80010001u;
        } else if(kind==9) throw new Exception("Compiler debugger reported a RIP event");
        Check(ContinueDebugEvent(pid,tid,status),"ContinueDebugEvent");
        if(rootExited&&active.Count==0)break;
      }
      Accounting accounting;long exitDeadline=clock.ElapsedMilliseconds+2000;
      do {
        Check(QueryInformationJobObject(job,1,out accounting,(uint)Marshal.SizeOf(typeof(Accounting)),IntPtr.Zero),"Read owned job accounting");
        if(accounting.active==0||accounting.total!=(uint)report.processes.Count)break;
        System.Threading.Thread.Sleep(10);
      } while(clock.ElapsedMilliseconds<exitDeadline);
      report.totalProcesses=accounting.total;report.activeProcesses=accounting.active;
      if(accounting.active!=0||accounting.total!=(uint)report.processes.Count||!rootExited)throw new Exception("Incomplete compiler descendant coverage");
      report.complete=true;
    } catch(Exception exception) {
      report.error=exception.ToString(); if(assigned)TerminateJobObject(job,1);else if(initial.process!=IntPtr.Zero)TerminateProcess(initial.process,1);
    } finally {
      if(initial.process!=IntPtr.Zero)CloseHandle(initial.process);if(initial.thread!=IntPtr.Zero)CloseHandle(initial.thread);
      foreach(IntPtr handle in new IntPtr[]{input,output,error,job})if(handle!=IntPtr.Zero)CloseHandle(handle);
      foreach(IntPtr pointer in new IntPtr[]{limits,eventBuffer,env})if(pointer!=IntPtr.Zero)Marshal.FreeHGlobal(pointer);
    }
    return report;
  }
}
'@
} finally {
    $env:TEMP = $auditTempParent
    $env:TMP = $auditTempParent
    $resolvedTemp = [IO.Path]::GetFullPath($auditTempPath)
    if ([IO.Path]::GetDirectoryName($resolvedTemp) -ne $auditTempParent -or [IO.Path]::GetFileName($resolvedTemp) -notmatch '^aware-compiler-audit-[0-9a-f]{32}$') { throw 'Auditor temporary cleanup escaped its parent' }
    if (([IO.File]::GetAttributes($resolvedTemp) -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw 'Auditor temporary cleanup found a redirection' }
    [IO.Directory]::Delete($resolvedTemp, $true)
}
$utf8 = New-Object System.Text.UTF8Encoding($false)
if ($request.mode -eq 'host') {
    $paths = [AwareCompilerAudit]::Host()
    [IO.File]::WriteAllText($request.output, (@{windows=$paths[0];system32=$paths[1]} | ConvertTo-Json -Compress), $utf8)
    exit 0
}
if ($request.mode -ne 'run') { throw 'Unsupported compiler audit mode' }
$parts = foreach ($property in ($request.environment.PSObject.Properties | Sort-Object Name)) {
    if ($property.Name.Contains('=') -or $property.Name.Contains([char]0) -or ([string]$property.Value).Contains([char]0)) { throw 'Invalid compiler environment block' }
    $property.Name + '=' + [string]$property.Value
}
$environment = ($parts -join [char]0) + [char]0 + [char]0
$policy = New-Object AwareCompilerAudit+StartupPolicy
$policy.identity = [string]$request.startupPolicy.identity
if ($null -ne $request.startupPolicy.deniedImage) {
    $policy.deniedImage = New-Object AwareCompilerAudit+DeniedImage
    foreach ($name in @('closure','relativePath','path','sha256','size','exitCode')) { $policy.deniedImage.$name = $request.startupPolicy.deniedImage.$name }
}
$result = [AwareCompilerAudit]::Run($request.executable, [string[]]$request.args, $request.cwd, $environment, [int]$request.timeoutMs, $policy)
$result | Add-Member -NotePropertyName identity -NotePropertyValue $request.identity
[IO.File]::WriteAllText($request.output, ($result | ConvertTo-Json -Depth 15), $utf8)
if (!$result.complete) { throw $result.error }
exit 0
