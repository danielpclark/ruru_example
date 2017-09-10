desc 'Run testsuite'
task :test do
  sh 'cargo build --release'
  sh 'ruby example.rb'
end

task default: :test
