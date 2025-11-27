defmodule Rps do
  # A for Rock, B for Paper, and C for Scissors
  # X for Rock, Y for Paper, and Z for Scissors

  # X -> lose, Y -> draw, Z - > win

  # 1 for Rock, 2 for Paper, and 3 for Scissors
  # 0 if you lost, 3 if the round was a draw, and 6 if you won

  # score -> 15
  def read(file) do
    file
    |> File.read!()
    |> String.split("\n")
    |> Enum.map(&String.split(&1, " "))
    |> Enum.filter(&length(&1) > 1)
    |> Enum.map(&game_on/1)
    |> Enum.sum()
  end

  def game_on(outcomes) do
    IO.puts(outcomes)
    case outcomes do
      ["A", "X"] -> 3 + 1
      ["A", "Y"] -> 6 + 2
      ["A", "Z"] -> 0 + 3
      ["B", "X"] -> 0 + 1
      ["B", "Y"] -> 3 + 2
      ["B", "Z"] -> 6 + 3
      ["C", "X"] -> 6 + 1
      ["C", "Y"] -> 0 + 2
      ["C", "Z"] -> 3 + 3
      _ -> 0
    end
  end
end
