defmodule Tree do
  @enforce_keys [:name]

  alias __MODULE__, as: T

  defstruct [
    :name,
    :fsize,
    :children,
    :parent
  ]

  def new(name) do
    %T{
      name: name,
      fsize: 0,
      children: [],
      parent: nil
    }
  end

  def head(%T{name: name}) do
    name
  end

  def add_dir(
    curr = %T{children: children},
    dir_name
  ) do
    %T{
      curr
      | children: [
        %T{
          name: dir_name,
          fsize: 0,
          children: [],
          parent: nil
        }
        | children
      ]
    }
  end

  def change_dir_down(
    curr = %T{children: children},
    dir_name
  ) do
    case Enum.find(children, &(&1.name == dir_name)) do
      nil ->
        curr

      dir ->
        dir
        |> Map.put(:parent, curr)
    end
  end

  def change_dir_up(%T{parent: parent}) do
    parent
  end

  def add_fsize(
    curr = %T{fsize: fsize},
    size
  ) do
    %T{
      curr
      | fsize: fsize + size
    }
  end

end
