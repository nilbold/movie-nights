# frozen_string_literal: true

require 'test_helper'

class TestMovieNights < Minitest::Test
  def test_version_number
    refute_nil ::MovieNights::VERSION
  end
end
