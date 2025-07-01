defmodule ZkArcadeWeb.ClaimController do
  require Logger
  use ZkArcadeWeb, :controller
  alias ZkArcade.VerifySignature

  plug :check_step
  @step 2

  def home(conn, _params) do
    # The home page is often custom made,
    # so skip the default app layout.

    render(conn, :home, layout: false)
  end

  def check_claimable(conn, %{"address" => address, "signature" => sig}) do
    conn = VerifySignature.call(conn, address, sig)

    if conn.assigns[:error] do
      conn |> assign(:error, "Failure in signature authentication")
      render(conn, :home, layout: false)
    else
      case ZkArcade.Accounts.create_wallet(%{address: String.downcase(address)}) do
        {:ok, wallet} ->
          Logger.info("Wallet creada: #{wallet.address}")

        {:error, changeset} ->
          Logger.error("Error al crear wallet: #{inspect(changeset.errors)}")

          conn
          |> assign(:error, "Hubo un problema al crear tu wallet")
          |> render(:home, layout: false)
      end

      render(conn, :home, layout: false)
    end
  end

  defp check_step(conn, _) do
    case Plug.Conn.get_session(conn, :step) do
      step when step in [1, 2]->  conn |> put_session(:step, @step)
      _ -> conn |> halt() |> redirect(to: "/")
    end
  end
end
