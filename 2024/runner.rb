require 'cli/ui'
require 'filewatcher'
require 'byebug'

module Runner
  def self.start 
    CLI::UI::StdoutRouter.enable
    puts CLI::UI.fmt "❄️🎄{{blue:Welcome to Advent of Code 2024!}}🎄❄️"

    CLI::UI::ask 'Which day would you like to run?' do |handler|
      (1..25).each do |day|
        handler.option int_to_written day do |day|
          puts CLI::UI.fmt "{{gray:Running day #{day}}}"
          generate_files day if !Dir.exist? "./day-#{day}"

          if solved?(day, 'one') && solved?(day, 'two')
            puts CLI::UI.fmt "{{green:Day #{day} already solved  ⭐⭐!}}"
            next
          end

          if solved? day, 'one'
            puts CLI::UI.fmt "{{green:Part one already solved}}"
            puts CLI::UI.fmt "{{gray:Running part two}}"
            run_part day, 'two'
          else
            puts CLI::UI.fmt "{{gray:Running part one}}"
            run_part day, 'one'
          end
        end
      end
    end
  end

  def self.int_to_written(day)
    {
      1 => 'one',
      2 => 'two',
      3 => 'three',
      4 => 'four',
      5 => 'five',
      6 => 'six',
      7 => 'seven',
      8 => 'eight',
      9 => 'nine',
      10 => 'ten',
      11 => 'eleven',
      12 => 'twelve',
      13 => 'thirteen',
      14 => 'fourteen',
      15 => 'fifteen',
      16 => 'sixteen',
      17 => 'seventeen',
      18 => 'eighteen',
      19 => 'nineteen',
      20 => 'twenty',
      21 => 'twenty-one',
      22 => 'twenty-two',
      23 => 'twenty-three',
      24 => 'twenty-four',
      25 => 'twenty-five'
    }[day]
  end

  def self.generate_files(day)
    source_folder = './template'
    destination_folder = "./day-#{day}"
    begin
      FileUtils.cp_r source_folder, destination_folder
      puts CLI::UI.fmt "{{green:Files generated for day #{day}}}"
    rescue Errno::ENOENT => e
      puts CLI::UI.fmt "{{red:#{e.messsage}}}"
    end
  end

  def self.solved?(day, part)
    File.exist? "./day-#{day}/part-#{part}-result"
  end

  def self.remove_results(day, part)
    File.delete "./day-#{day}/part-#{part}-result"
  end

  def self.confirm(day, part)
    CLI::UI::Prompt.confirm("Did it pass?").then do |confirmed|
      if confirmed
        if part == 'one'
          puts CLI::UI.fmt "{{green:Great job! ⭐ Now let's run part two.}}"
          run_part day, 'two'
        else
          puts CLI::UI.fmt "{{green:Awesome! ⭐⭐ Now it's time for a 🍪.}}" 
        end
      else
        puts CLI::UI.fmt "{{red:💩}}"
        remove_results day, part
      end
    end
  end

  def self.run_part(day, part)
    run = -> do
      require_relative_and_reload "day-#{day}/part-#{part}.rb"
      klass = Object.const_get("Part#{part.capitalize}").new
      result = klass.assert
      puts CLI::UI.fmt "{{v}} #{result}"
      result = klass.solve
      puts CLI::UI.fmt "{{*}} #{result}"
      File.write("day-#{day}/part-#{part}-result", result)
      confirm day, part
    end
    begin
      run.call
    rescue PuzzleSolutionError, NotImplementedError => e
      puts CLI::UI.fmt "{{x}} #{e.message}"
      Filewatcher.new(["./day-#{day}"]).watch do |filename, event|
        puts CLI::UI.fmt "{{gray:#{filename} #{event}...}}"
        begin
          run.call
        rescue PuzzleSolutionError, NotImplementedError => e 
          puts CLI::UI.fmt "{{x}} #{e.message}"
        end
      end
    end
  rescue => e
    puts CLI::UI.fmt "{{red:#{e.message}}}"
    puts CLI::UI.fmt "{{red:#{e.backtrace.join("\n")}}"

  end

  def self.require_relative_and_reload(file)
    load file if $LOADED_FEATURES.include? File.expand_path file
    require_relative file
  end
end

Runner.start
