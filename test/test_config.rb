# frozen_string_literal: true

require 'test_helper'

class TestConfig < Minitest::Test
  def test_valid_params
    MovieNights.configure do |config|
      config.token = '0000'
    end
  end

  def test_missing_params
    assert_raises MovieNights::Error do
      MovieNights.configure { |c| } # rubocop:disable Lint/EmptyBlock
    end
  end
end
