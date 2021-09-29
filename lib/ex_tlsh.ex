defmodule ExTlsh do
  use Rustler, otp_app: :ex_tlsh, crate: "ex_tlsh"

  @spec hash(binary) :: binary
  def hash(_b64), do: error()

  defp error(), do: :erlang.nif_error(:nif_not_loaded)

end
