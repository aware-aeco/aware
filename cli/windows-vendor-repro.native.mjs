// Required Windows integration: real Cargo, a dependency outside the build root, and native bytes.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, relative, resolve } from 'node:path';
import { createCargoBuild, assertVerboseCargoProof, verifyExtractedInputs, runningInputFiles,
  normalizeBuildText, WINDOWS_LOGICAL_RUST_FLAGS } from './build-windows-internal-repro.mjs';
import { prepareNativeCompiler, nativeVersionProof, nativeToolsProof, nativeBootstrapProof, nativeLifecycleProof } from './windows-compiler-native-fixture.mjs';
import { beneath, loaderObservedWindows, compilerStartupPolicy, prepareNativeArchiveAdapter, nativeArchiveAdapterRecord, verifyNativeArchiveAdapter } from './windows-compiler-closure.mjs';

assert.equal(process.platform, 'win32', 'native repro gate requires Windows');
assert.equal(process.arch, 'x64', 'native repro gate requires x64');
function run(tool, args, options = {}) {
  const result = spawnSync(tool, args, { encoding: 'utf8', windowsHide: true, timeout: 120000, maxBuffer: 16 * 1024 * 1024, ...options });
  assert.equal(result.error, undefined, String(result.error));
  assert.equal(result.status, 0, `${tool} ${args.join(' ')}\n${result.stdout}\n${result.stderr}`);
  return `${result.stdout ?? ''}${result.stderr ?? ''}`;
}
const root = mkdtempSync(join(tmpdir(), 'aware vendor repro '));
// Only this fresh empty fixture changes its compression attribute. Two uncompressed
// compiler copies fit the native gate's budget and avoid repeated decompression on
// every mandatory byte check; source installations and retained evidence are untouched.
const hash = path => createHash('sha256').update(readFileSync(path)).digest('hex');
const records = [];
let completed = false;
function physicalLeaks(directory, roots) {
  return readdirSync(directory, {withFileTypes:true}).flatMap(entry => {
    const file=join(directory,entry.name);
    if(entry.isDirectory()) return physicalLeaks(file,roots);
    if(!/\.(exe|dll|lib|rlib|rmeta)$/i.test(entry.name)) return [];
    const bytes=readFileSync(file), texts=[bytes.toString('latin1'),bytes.toString('utf8'),bytes.toString('utf16le'),bytes.subarray(1).toString('utf16le')].map(x=>x.toLowerCase());
    return roots.some(root=>[root,root.replaceAll('\\','/'),root.replaceAll('\\','\\\\')].some(p=>texts.some(text=>text.includes(p.toLowerCase())))) ? [file] : [];
  });
}
function compiledInventory(directory, root=directory) {
  return readdirSync(directory,{withFileTypes:true}).sort((a,b)=>a.name.localeCompare(b.name)).flatMap(entry=>{
    const file=join(directory,entry.name);if(entry.isDirectory())return compiledInventory(file,root);
    if(!/\.(exe|dll|lib|rlib|rmeta)$/i.test(entry.name))return [];
    const bytes=readFileSync(file);return [{path:relative(root,file).replaceAll('\\','/'),size:bytes.length,sha256:hash(file)}];
  });
}
function archiveMembers(file) {
  const bytes=readFileSync(file);assert.equal(bytes.subarray(0,8).toString(),'!<arch>\n');let pos=8,longnames;const rows=[];
  while(pos<bytes.length){const header=bytes.subarray(pos,pos+60),name=header.subarray(0,16).toString().trim(),size=Number(header.subarray(48,58).toString());
    assert.equal(header.subarray(58).toString(),'`\n');assert.ok(Number.isSafeInteger(size)&&size>=0&&pos+60+size<=bytes.length);
    const data=bytes.subarray(pos+60,pos+60+size);if(name==='//')longnames=data;else if(name!=='/')rows.push({name,data});pos+=60+size+(size%2);
  }
  return rows.map(row=>{let name=row.name;if(/^\/\d+$/.test(name)){assert.ok(longnames);const offset=Number(name.slice(1)),end=longnames.indexOf(0,offset);assert.ok(end>=offset);name=longnames.subarray(offset,end).toString();}
    return {name,size:row.data.length,sha256:createHash('sha256').update(row.data).digest('hex')};});
}
try {
  run(join(loaderObservedWindows().system32, 'compact.exe'), ['/U', root]);
  nativeBootstrapProof(root);
  nativeLifecycleProof(root);
  for (const side of ['a', 'b']) {
    const base = join(root, `builder ${side} Łódź 😀 with a supported long source location`), work = join(base, 'work'), source = join(work, 'source'), crateRoot = join(source, 'cli');
    const closure = join(base, 'sealed cache'), vendor = join(closure, 'vendor'), dependency = join(vendor, 'path-probe'), macro = join(vendor,'path-macro');
    const cargoHome = join(work, 'cargo-home'), tempRoot = join(work, 'temp');
    for (const dir of [join(crateRoot, 'src'), join(dependency, 'src'), join(macro,'src'), cargoHome, tempRoot]) mkdirSync(dir, { recursive: true });
    writeFileSync(join(dependency, 'Cargo.toml'), '[package]\nname="path-probe"\nversion="0.1.0"\nedition="2021"\n');
    writeFileSync(join(dependency, 'src', 'lib.rs'), 'pub fn origin() -> &\'static str { file!() }\n');
    writeFileSync(join(dependency, '.cargo-checksum.json'), JSON.stringify({ package: '0'.repeat(64), files: {
      'Cargo.toml': hash(join(dependency, 'Cargo.toml')), 'src/lib.rs': hash(join(dependency, 'src', 'lib.rs')),
    } }));
    writeFileSync(join(macro,'Cargo.toml'),'[package]\nname="path-macro"\nversion="0.1.0"\nedition="2021"\n[lib]\nproc-macro=true\n');
    writeFileSync(join(macro,'src','lib.rs'),'extern crate proc_macro;\n#[proc_macro]\npub fn origin(_: proc_macro::TokenStream) -> proc_macro::TokenStream { format!("{:?}", file!()).parse().unwrap() }\n');
    writeFileSync(join(macro,'.cargo-checksum.json'),JSON.stringify({package:'1'.repeat(64),files:{'Cargo.toml':hash(join(macro,'Cargo.toml')),'src/lib.rs':hash(join(macro,'src','lib.rs'))}}));
    writeFileSync(join(crateRoot, 'Cargo.toml'), '[package]\nname="vendor-repro-probe"\nversion="0.1.0"\nedition="2021"\n[dependencies]\npath-probe="=0.1.0"\npath-macro="=0.1.0"\n[build-dependencies]\npath-probe="=0.1.0"\n[profile.release]\ndebug=0\n');
    writeFileSync(join(crateRoot, 'src', 'main.rs'), 'extern "C" { fn native_one() -> i32; fn native_two() -> i32; }\nfn main() { println!("{}", path_probe::origin()); println!("{}", path_macro::origin!()); /* SAFETY: fixture functions are linked from the two audited C objects, take no arguments and return integers. */ println!("{}", unsafe { native_one()+native_two() }); }\n');
    const native = prepareNativeCompiler({ base, work, source, closure, side, run });
    const { compiler, privateClosure, audit } = native;
    symlinkSync(crateRoot, join(crateRoot, 'archive-junction'), 'junction');
    const denied = compilerStartupPolicy(compiler).deniedImage;
    // Always exercise inherited heap configuration, even when optional telemetry is absent.
    // The negative child loses the flag AFTER the valid root request, so it tests
    // descendant inheritance rather than merely tripping the launcher's preflight.
    writeFileSync(join(crateRoot, 'build.rs'), `
#[link(name = "kernel32")]
extern "system" { fn IsDebuggerPresent() -> i32; }
fn main() {
    assert_eq!(std::env::var("_NO_DEBUG_HEAP").as_deref(), Ok("1"), "AWARE_HEAP_SETTING_MISSING:_NO_DEBUG_HEAP");
    // SAFETY: documented zero-argument Win32 query; no pointers or borrowed memory.
    assert_ne!(unsafe { IsDebuggerPresent() }, 0, "auditor must remain attached");
    if std::env::args().nth(1).as_deref() == Some("heap-child") {
        println!("AWARE_HEAP_CHILD_OK");
        return;
    }
    println!("cargo:warning=AWARE_HOST_PATH:{}", path_probe::origin());
    let executable = std::env::current_exe().unwrap();
    let mut positive = std::process::Command::new(&executable);
    positive.arg("heap-child").stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped());
    let child = positive.spawn().unwrap(); let positive_pid = child.id();
    let good = child.wait_with_output().unwrap();
    assert_eq!(good.status.code(), Some(0));
    assert_eq!(String::from_utf8_lossy(&good.stdout).trim(), "AWARE_HEAP_CHILD_OK");
    let mut negative = std::process::Command::new(&executable);
    negative.arg("heap-child").env_remove("_NO_DEBUG_HEAP").stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped());
    let child = negative.spawn().unwrap(); let negative_pid = child.id();
    let bad = child.wait_with_output().unwrap();
    assert_eq!(bad.status.code(), Some(101), "missing inherited flag must panic");
    assert!(String::from_utf8_lossy(&bad.stderr).contains("AWARE_HEAP_SETTING_MISSING:_NO_DEBUG_HEAP"));
    assert!(!String::from_utf8_lossy(&bad.stdout).contains("AWARE_HEAP_CHILD_OK"));
    ${denied ? `let telemetry = std::path::PathBuf::from(std::env::var("VCINSTALLDIR").unwrap()).join("bin/vctip.exe");
    let status = std::process::Command::new(telemetry).status().expect("private telemetry child");
    assert_eq!(status.code().unwrap() as u32, ${denied.exitCode}u32);` : ''}
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut objects = Vec::new();
    for (name, value) in [("one",1),("two",2)] {
        let dir=out.join(name); std::fs::create_dir_all(&dir).unwrap();
        let source=dir.join("probe.c"); let object=dir.join("probe.o");
        std::fs::write(&source, format!("int native_{}(void) {{ return {}; }}", name,value)).unwrap();
        let result=std::process::Command::new(std::env::var("CC").unwrap()).args(["/nologo","/MT","/O2","/c"]).arg(format!("/Fo{}",object.display())).arg(source).output().unwrap();
        assert!(result.status.success(),"native compiler: {:?}",result);
        objects.push(object);
    }
    let library=out.join("nativeprobe.lib");
    for (index,object) in objects.iter().enumerate() {
        let mut command=std::process::Command::new(std::env::var("AR").unwrap());
        command.arg(format!("-out:{}",library.display())).arg("-nologo");
        if index>0 { command.arg(&library); }
        let result=command.arg(object).output().unwrap(); assert!(result.status.success(),"native librarian: {:?}",result);
    }
    let before=std::fs::read(&library).unwrap();
    for alias in ["COM¹", "COM²", "COM³", "LPT¹", "LPT²", "LPT³", "CONIN$", "CONOUT$", "CLOCK$", "COM1 "] {
        for args in [vec![format!("/OUT:{}",out.join(format!("{}.lib",alias)).display()),objects[0].display().to_string()],
            vec![format!("/OUT:{}",library.display()),out.join(format!("{}.obj",alias)).display().to_string()]] {
            let refused=std::process::Command::new(std::env::var("AR").unwrap()).args(args).output().unwrap();
            assert_eq!(refused.status.code(),Some(2));
            assert!(String::from_utf8_lossy(&refused.stderr).contains("reserved device component"));
            assert_eq!(std::fs::read(&library).unwrap(),before,"device refusal must not mutate the archive");
        }
    }
    let output_arg=format!("/OUT:{}",library.display());
    let object_arg=objects[0].display().to_string();
    let other=out.join("other.lib"); std::fs::copy(&library,&other).unwrap();
    let cwd=std::env::current_dir().unwrap();
    let response=cwd.join("@payload.obj"); let option=cwd.join("-payload.obj");
    let escaped=cwd.join("must-not-escape.lib");
    std::fs::write(&response,format!("/OUT:{} {}",escaped.display(),objects[0].display())).unwrap();
    std::fs::copy(&objects[0],&option).unwrap();
    for args in [vec![output_arg.clone(),"@inputs.rsp".into()],
        vec![output_arg.clone(),response.display().to_string()],
        vec![output_arg.clone(),option.display().to_string()],
        vec![output_arg.clone(),cwd.join("archive-junction/-payload.obj").display().to_string()],
        vec![output_arg.clone(),"/UNKNOWN".into(),object_arg.clone()],
        vec![output_arg.clone(),"C:drive.obj".into()],
        vec![output_arg.clone(),output_arg.clone(),object_arg.clone()],
        vec![output_arg.clone(),other.display().to_string(),object_arg.clone()],
        vec![format!("/OUT:{}/../../escape.lib",out.display()),object_arg.clone()]] {
        let reparse_case=args.iter().any(|arg|arg.contains("archive-junction"));
        if reparse_case { assert!(cwd.join("archive-junction/-payload.obj").is_file()); }
        let refused=std::process::Command::new(std::env::var("AR").unwrap()).args(args).output().unwrap();
        assert_eq!(refused.status.code(),Some(2));
        assert!(String::from_utf8_lossy(&refused.stderr).contains("AWARE_NATIVE_ARCHIVE_REFUSED"));
        if reparse_case { assert!(String::from_utf8_lossy(&refused.stderr).contains("reparse path is forbidden")); }
        assert_eq!(std::fs::read(&library).unwrap(),before,"refusal must not mutate the archive");
        assert!(!escaped.exists(),"converted filename must never inject librarian options");
    }
    println!("cargo:rustc-link-search=native={}",out.display()); println!("cargo:rustc-link-lib=static=nativeprobe");
    println!("cargo:warning=AWARE_HEAP_PROOF parent={} positive={} negative={}", std::process::id(), positive_pid, negative_pid);
}
`);
    const { cargoVendor, env, args } = createCargoBuild({ compiler, workRoot: work, sourceRoot: source, cargoHome, cargoClosure: privateClosure, tempRoot });
    nativeVersionProof({ native, env });
    const adapter=prepareNativeArchiveAdapter(work), flags=env.CARGO_ENCODED_RUSTFLAGS.split('\x1f');
    audit('rustc',[adapter.source,'--crate-name','aware_native_archive_adapter','--edition=2021','-C','opt-level=2','-C','debuginfo=0','-C',`linker=${compiler.tools.link}`,...flags,'-o',adapter.executable],env,'native-archive-adapter-build');
    const adapterRecord=nativeArchiveAdapterRecord(base);
    const config = args.flatMap((arg, index) => arg === '--config' ? [arg, args[index + 1]] : []);
    audit('cargo', ['generate-lockfile', '--manifest-path', join(crateRoot, 'Cargo.toml'), '--offline', ...config], env, 'cargo-lock');
    const verbose = audit('cargo', args, env, 'cargo-build');
    assert.ok(verbose.includes('AWARE_HOST_PATH:<cargo-vendor>'), 'real host build dependency receives its specific remap');
    const heapProof = /cargo:warning=AWARE_HEAP_PROOF parent=(\d+) positive=(\d+) negative=(\d+)/.exec(verbose);
    assert.ok(heapProof, 'audited build-script parent authenticated both inherited-heap outcomes');
    const heapAudit = JSON.parse(readFileSync(join(native.evidence, 'cargo-build-audit.local.json'), 'utf8'));
    verifyNativeArchiveAdapter(adapterRecord,heapAudit,base);
    const parentCandidates = heapAudit.processes.filter(process => process.pid === Number(heapProof[1])
      && /[\\/]vendor-repro-probe-[^\\/]+[\\/]build-script-build\.exe$/i.test(process.path));
    assert.equal(parentCandidates.length, 1, 'one exact build-script parent lifetime');
    const parent = parentCandidates[0]; assert.equal(parent.exitCode, 0);
    for (const [pid, exitCode] of [[Number(heapProof[2]), 0], [Number(heapProof[3]), 101]]) {
      const children = heapAudit.processes.filter(process => process.pid === pid && process.path === parent.path && process.exitCode === exitCode
        && process.startEvent > parent.startEvent && process.exitEvent < parent.exitEvent);
      assert.equal(children.length, 1, 'one exact nested child lifetime within the observed parent');
      assert.equal(children[0].exitCode, exitCode, 'audited child exit matches its authenticated outcome');
      const images = heapAudit.images.filter(image => image.instance === children[0].instance && image.kind === 'process');
      assert.equal(images.length, 1); assert.equal(images[0].path, parent.path);
      assert.equal(images[0].sha256, hash(parent.path), 'nested child image binds the actual build script');
    }
    if (denied) {
      const proof = JSON.parse(readFileSync(join(native.evidence, 'cargo-build-audit.local.json'), 'utf8'));
      assert.ok(proof.processes.some(process => process.action === 'blocked-telemetry' && process.exitCode === denied.exitCode), 'real private telemetry descendant was denied and its exit observed');
    }
    assert.equal(normalizeBuildText(flags.join(' '), [[work, '<work>'], [source, '<source>'],
      [cargoHome, '<cargo-home>'], [cargoVendor, '<cargo-vendor>']]), WINDOWS_LOGICAL_RUST_FLAGS,
    'the receipt records the complete logical compiler arguments');
    assertVerboseCargoProof(`${args.join(' ')}\n${verbose}`, flags, compiler.tools.rustc);
    const executable = join(work, 'cargo-target', 'release', 'vendor-repro-probe.exe');
    const origin = run(executable, [], { env }).trim().replaceAll('\\', '/');
    assert.equal(origin.replaceAll('\r',''), '<cargo-vendor>/path-probe/src/lib.rs\n<cargo-vendor>/path-macro/src/lib.rs\n3');
    const goodHash = hash(executable);
    assert.deepEqual(physicalLeaks(join(work,'cargo-target'),[base,vendor,compiler.root]),[], 'host dependencies, proc macros, native archives and final images contain no physical roots');
    const compiled=compiledInventory(join(work,'cargo-target'));
    const libraryRecord=compiled.find(record=>record.path.endsWith('/nativeprobe.lib'));assert.ok(libraryRecord);
    const library=join(work,'cargo-target',...libraryRecord.path.split('/')),members=archiveMembers(library);
    assert.deepEqual(members.map(member=>member.sha256).sort(),['one','two'].map(name=>hash(join(dirname(library),name,'probe.o'))).sort(), 'archive preserves both same-basename object payloads exactly');
    assert.ok(members.every(member=>!member.name.includes(base)&&member.name.includes('cargo-target')), 'archive names retain relative directories, not physical roots');

    // Exercise both argument spellings directly, in addition to Cargo's native dependency invocation.
    for (const [index, spelling] of [join(cargoVendor, 'path-probe', 'src', 'lib.rs'), join(cargoVendor, 'path-probe', 'src', 'lib.rs').replaceAll('\\', '/')].entries()) {
      const probeSource = join(source, 'spelling.rs');
      writeFileSync(probeSource, `#[path=${JSON.stringify(spelling)}] mod dependency;\nfn main() {println!("{}", dependency::origin());}\n`);
      const output = join(work, 'spelling.exe');
      audit('rustc', [probeSource, '--crate-name', 'spelling_probe', '-O', '-C', 'debuginfo=0', '-C', `linker=${compiler.tools.link}`, ...flags, '-o', output], env, `rust-spelling-${index}`);
      assert.equal(run(output, [], { env }).trim().replaceAll('\\', '/'), '<cargo-vendor>/path-probe/src/lib.rs');
    }

    const badFlags = flags.filter(flag => !flag.endsWith('=<cargo-vendor>'));
    assert.equal(flags.length - badFlags.length, 2, 'mutation removes BOTH Windows vendor spellings');
    const badEnv = { ...env, CARGO_ENCODED_RUSTFLAGS: badFlags.join('\x1f'), CARGO_TARGET_DIR: join(work, 'cargo-target', 'mutated-target') };
    audit('cargo', args, badEnv, 'cargo-red-control');
    const badExecutable = join(work, 'cargo-target', 'mutated-target', 'release', 'vendor-repro-probe.exe');
    const badOrigin = run(badExecutable, [], { env: badEnv }).trim().replaceAll('\\', '/');
    assert.equal(badOrigin.replaceAll('\r',''), '<work>/cargo-closure/vendor/path-probe/src/lib.rs\n<work>/cargo-closure/vendor/path-macro/src/lib.rs\n3', 'mutation exposes only the broad work remap');
    assert.notEqual(hash(badExecutable), goodHash, 'mutation changes the executable');
    const cHash = nativeToolsProof({ native, env, run });
    const hostEnv={...env,CARGO_TARGET_DIR:join(work,'cargo-target','host-negative')};
    const hostArgs=[...args,'--target','x86_64-pc-windows-msvc'];
    const hostLog=audit('cargo',hostArgs,hostEnv,'cargo-host-red-control');
    assert.throws(()=>assertVerboseCargoProof(hostArgs.join(' ')+'\n'+hostLog,flags,compiler.tools.rustc),/omitted/);
    assert.ok(physicalLeaks(join(work,'cargo-target','host-negative'),[base]).length>0,'explicit target mutation exposes real host paths');
    records.push({ side, origin, goodHash, badOrigin, badHash: hash(badExecutable), cHash, adapterHash:hash(adapter.executable),compiled });
    native.finish(records.at(-1));
    assert.ok(beneath(base, root) && resolve(base) !== resolve(root));
  }
  assert.equal(records[0].goodHash, records[1].goodHash, 'independent Cargo builds are byte-identical');
  assert.equal(records[0].cHash, records[1].cHash, 'independent native C executables are byte-identical');
  assert.equal(records[0].adapterHash, records[1].adapterHash, 'independent source-bound archive adapters are byte-identical');
  assert.deepEqual(records[0].compiled,records[1].compiled,'complete compiled host/native artifact inventories match across roots');
  // The private vendor now lives under work: removing its specific remaps changes both
  // executables to the same wrong logical path, rather than exposing the original cache root.
  for (const record of records) assert.notEqual(record.badHash, record.goodHash);

  // Real Git bundle materialization must not let an old runner build a different source script.
  const repo = join(root, 'source authority'), extracted = join(root, 'extracted source');
  mkdirSync(join(repo, 'cli'), { recursive: true }); mkdirSync(join(repo, 'cli-connection-reader'));
  const oldRoot = join(root, 'old source'); mkdirSync(join(oldRoot, 'cli'), { recursive: true }); mkdirSync(join(oldRoot, 'cli-connection-reader'));
  const script = join(repo, 'cli', 'build-windows-internal-repro.mjs'), running = join(oldRoot, 'cli', 'build-windows-internal-repro.mjs');
  for (const builder of [script, running]) for (const [id, path] of Object.entries(runningInputFiles(builder))) writeFileSync(path, id);
  writeFileSync(join(repo, 'cli', 'Cargo.lock'), 'lock'); writeFileSync(join(repo, 'cli-connection-reader', 'package-lock.json'), '{}');
  const git = 'git';
  run(git, ['init', repo]); run(git, ['-C', repo, 'add', '.']);
  const commitArgs = ['-c', 'user.name=Repro fixture', '-c', 'user.email=repro@example.invalid', '-c', 'commit.gpgsign=false', '-c', 'core.hooksPath=NUL', 'commit', '-qm'];
  run(git, ['-C', repo, ...commitArgs, 'old fixture']);
  const inputs = { 'aware-cargo-lock': hash(join(repo, 'cli', 'Cargo.lock')), 'reader-package-lock': hash(join(repo, 'cli-connection-reader', 'package-lock.json')),
    ...Object.fromEntries(Object.entries(runningInputFiles(script)).map(([id, path]) => [id, hash(path)])) };
  writeFileSync(script, 'new builder'); run(git, ['-C', repo, 'add', '.']); run(git, ['-C', repo, ...commitArgs, 'new fixture']);
  const bundle = join(root, 'source.bundle'); run(git, ['-C', repo, 'bundle', 'create', bundle, 'HEAD']); run(git, ['clone', bundle, extracted]);
  assert.throws(() => verifyExtractedInputs(extracted, inputs, running), /extracted source/);
  inputs['builder-script'] = hash(join(extracted, 'cli', 'build-windows-internal-repro.mjs'));
  assert.throws(() => verifyExtractedInputs(extracted, inputs, running), /extracted source/);
  console.log(`Windows Cargo vendor-path repro passed: 2 byte-identical executables, both path spellings, spaced paths, two red mutations, old-runner/new-bundle refusal. ${records[0].goodHash}`);
  completed = true;
} finally {
  // This test owns the unique temporary directory it created above.
  if (!beneath(root, tmpdir()) || resolve(root) === resolve(tmpdir())) throw new Error('test cleanup escaped its temporary parent');
  if (completed) rmSync(root, { recursive: true, force: true });
  else console.error(`Native failure evidence retained: ${root}`);
}
