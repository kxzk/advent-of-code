defmodule Caloriecnt do
  def get_calories(file) do
    file
    |> File.stream!()
    |> Stream.map(&String.replace(&1, "\n", ""))
    |> Stream.map(&convert_value/1)
    |> Enum.to_list()
    |> Enum.chunk_by(&(&1 == -1))
    |> Enum.map(&Enum.sum/1)
  end

  def get_top_k(cal_sums, k) do
    cal_sums
    |> Enum.sort()
    |> Enum.reverse()
    |> Enum.take(k)
  end

  def get_sum(cal_sums) do
    cal_sums
    |> Enum.sum()
  end

  def convert_value(cal_str) do
    case cal_str do
      "" -> -1
      _ -> String.to_integer(cal_str)
    end
  end

end
