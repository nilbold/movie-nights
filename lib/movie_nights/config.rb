# frozen_string_literal: true

# provides configuration options to the MovieNights module
module MovieNights
  # movie nights configuration
  class Config
    attr_accessor :token
  end

  def self.config
    @config
  end

  def self.configure
    c = Config.new
    yield c

    raise Error, 'discord token missing from configuration' if c.token.nil?

    @config = c.freeze
  end
end
