# frozen_string_literal: true

require 'rake'

require_relative 'lib/movie_nights/version'

Gem::Specification.new do |s|
  s.name = 'movie-nights'
  s.version = MovieNights::VERSION
  s.authors = ['nil']
  s.email = ['nil@kobold.dev']

  s.summary = 'movie nights discord bot'
  s.homepage = 'https://github.com/nilbold/movie-nights/'
  s.license = 'MIT'

  s.required_ruby_version = '>= 2.6.0'

  s.metadata = {
    'homepage_uri' => s.homepage,
    'source_code_uri' => 'https://github.com/nilbold/movie-nights/',
    'allowed_push_host' => '',
    'rubygems_mfa_required' => 'true',
  }

  s.files = FileList['lib/**/*.rb']
  s.bindir = 'exe'
  s.executables = s.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  s.require_paths = ['lib']

  s.add_dependency 'discordrb', '~> 3.5'
end
