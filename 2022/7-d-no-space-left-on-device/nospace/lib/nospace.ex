defmodule Nospace do
  @moduledoc """
  Documentation for `Nospace`.
  """

  @doc """
  Reads the `input.txt` file and converts the data
  into a format that is conducive to solving the problem.

  Specifically, it tokenizes the input so that it is
  easy to split the string and get the necessary data
  for building the tree.

  Returns `List[strings]`.
  """
  def read_and_fmt(file) do
    file
    |> File.read!()
    |> String.replace("\n", " ", global: true)
    |> String.split(~r{.*?(?=\$|dir|(?<![#\d])\d+|$)}, include_captures: true)
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.replace(&1, "$ ", ""))
    |> Enum.filter(&(String.length(&1) > 0))
    |> Enum.filter(&(&1 != "ls"))
    # |> Enum.each(&IO.puts/1)
    # |> Enum.each(&handle_token/1)
  end

  # def build_tree(lst_tok) do
    # # create a node from first token and add to tree
    # tree = Tree.new(Node.new(List.last(String.split(List.first(lst_tok), " "))))
    # tree.root
    # |> IO.puts
  # end
  # def handle_token(lst_tok) do
    # Enum.each(lst_tok, fn tok ->
      # curr_node = nil
      # case String.split(tok, " ") do
        # # create root node
        # ["cd", "/"] ->
          # root_node = TreeNode.new("/")
          # curr_node = root_node
        # # go up a dir, change to curr node
        # ["cd", ".."] ->
          # curr_node = curr_node.parent
        # # find child node, go down a dir
        # ["cd", dir] ->
          # curr_node = Enum.find(curr_node.children, &(&1.name == dir))
        # # add new node w/ parent as curr node
        # ["dir", name] ->
          # new_node = TreeNode.new(name, curr_node)
          # curr_node.children = [new_node | curr_node.children]
        # # add filesize to current dir
        # [filesize, _] ->
          # curr_node.size + String.to_integer(filesize)
        # _ -> IO.puts("error")
      # end
    # end)
  # end
end
