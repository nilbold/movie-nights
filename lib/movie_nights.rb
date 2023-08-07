# frozen_string_literal: true

require 'movie_nights/version'
require 'movie_nights/config'

require 'discordrb'

# the movie nights discord bot
module MovieNights
  class Error < StandardError; end

  def self.run
    bot = Discordrb::Bot.new token: config.token

    bot.run
  end
end
