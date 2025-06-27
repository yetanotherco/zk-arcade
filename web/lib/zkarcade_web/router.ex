defmodule ZkArcadeWeb.Router do
  use ZkArcadeWeb, :router
  # import ZkArcadeWeb.Plugs

  # https://furlough.merecomplexities.com/elixir/phoenix/security/2021/02/26/content-security-policy-configuration-in-phoenix.html

  @host Application.compile_env(:zkarcade, [ZkArcadeWeb.Endpoint, :url, :host], "localhost")

  @content_security_policy (case Mix.env() do
                              :prod ->
                                "default-src 'self' 'unsafe-inline';" <>
                                "connect-src wss://#{@host};" <>
                                "img-src 'self' https://w3.org http://raw.githubusercontent.com https://cdn.prod.website-files.com blob: data:;"
                              _ ->
                                "default-src 'self' 'unsafe-eval' 'unsafe-inline';" <>
                                  "connect-src ws://#{@host}:*;" <>
                                  "img-src * blob: data:;" <>
                                  "font-src data:;"
                            end)

  pipeline :browser do
    plug :accepts, ["html"]
    plug :fetch_session
    plug :fetch_live_flash
    # plug :load_theme_cookie_in_session
    plug :put_root_layout, html: {ZkArcadeWeb.Layouts, :root}
    plug :protect_from_forgery
    plug :put_secure_browser_headers, %{"content-security-policy" => @content_security_policy}
  end

  pipeline :api do
    plug CORSPlug, origin: "*"
    plug :accepts, ["json"]
  end

  scope "/", ZkArcadeWeb do
    pipe_through :browser

    live "/", PageLive, :home
  end

  # To Enable LiveDashboard: (only enable behind auth)
  # if Application.compile_env(:zkarcade, :dev_routes) do
  #   # If you want to use the LiveDashboard in production, you should put
  #   # it behind authentication and allow only admins to access it.
  #   # If your application does not have an admins-only section yet,
  #   # you can use Plug.BasicAuth to set up some basic authentication
  #   # as long as you are also using SSL (which you should anyway).
  #   import Phoenix.LiveDashboard.Router

  #   scope "/dev" do
  #     pipe_through :browser

  #     live_dashboard "/dashboard", metrics: ZkArcadeWeb.Telemetry
  #   end
  # end
end
