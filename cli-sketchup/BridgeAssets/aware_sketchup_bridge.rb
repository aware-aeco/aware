# aware_sketchup_bridge.rb — loader for the AWARE SketchUp Bridge.
#
# This is the file SketchUp auto-loads from
#   %APPDATA%\SketchUp\SketchUp <year>\SketchUp\Plugins\
# at every startup. It registers a SketchupExtension whose loader path points
# at aware_sketchup_bridge/core.rb where the actual TCP listener lives.
#
# The bridge speaks length-prefixed JSON over loopback TCP to
# aware-sketchup.exe so the AWARE runtime can shell ad-hoc Ruby into the
# live SketchUp document.
#
# BRIDGE_VERSION below is read by aware-sketchup --bridge-status to detect
# stale installs.

require 'sketchup.rb'
require 'extensions.rb'
require 'fileutils'
require 'tmpdir'
require 'time'

module AwareSketchupBridge
  EXTENSION_NAME = 'AWARE SketchUp Bridge'.freeze
  BRIDGE_VERSION = '0.34.0'.freeze

  # Diagnostic breadcrumb: log that the loader file ran at all. Helps
  # distinguish "SketchUp didn't load our .rb" from "extension registered but
  # didn't start". Lands at %TEMP%/aware-sketchup/boot-<pid>.log.
  begin
    diag_dir  = ENV['AWARE_SKETCHUP_DISCOVERY_DIR']
    diag_dir  = File.join(Dir.tmpdir, 'aware-sketchup') if diag_dir.nil? || diag_dir.empty?
    FileUtils.mkdir_p(diag_dir)
    File.open(File.join(diag_dir, "boot-#{Process.pid}.log"), 'a') do |f|
      f.puts("[#{Time.now.iso8601}] loader entered: #{__FILE__}")
    end
  rescue StandardError
    # Never block SketchUp boot on logging.
  end

  unless file_loaded?(__FILE__)
    here   = File.dirname(__FILE__)
    loader = File.join(here, 'aware_sketchup_bridge', 'core')
    ex = SketchupExtension.new(EXTENSION_NAME, loader)
    ex.description = 'TCP bridge from aware-sketchup.exe into the live SketchUp document.'
    ex.version     = BRIDGE_VERSION
    ex.copyright   = 'AWARE 2026'
    ex.creator     = 'AWARE'
    Sketchup.register_extension(ex, true)
    file_loaded(__FILE__)

    begin
      diag_dir2 = ENV['AWARE_SKETCHUP_DISCOVERY_DIR']
      diag_dir2 = File.join(Dir.tmpdir, 'aware-sketchup') if diag_dir2.nil? || diag_dir2.empty?
      File.open(File.join(diag_dir2, "boot-#{Process.pid}.log"), 'a') do |f|
        f.puts("[#{Time.now.iso8601}] extension registered, load_on_start=true")
      end
    rescue StandardError
    end
  end
end
