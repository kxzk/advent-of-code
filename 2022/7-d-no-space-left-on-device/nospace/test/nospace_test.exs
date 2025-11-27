defmodule NospaceTest do
  use ExUnit.Case
  doctest Nospace

  test "greets the world" do
    assert Nospace.hello() == :world
  end
end
