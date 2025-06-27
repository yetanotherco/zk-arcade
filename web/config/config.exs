# This file is responsible for configuring your application
# and its dependencies with the aid of the Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
import Config

config :zkarcade,
  batcher_submission_gas_cost: 125000,
  aggregator_gas_cost: 330000,
  additional_submission_gas_cost_per_proof: 2000,
  aggregator_fee_percentage_multiplier: 125,
  percentage_divider: 100

config :zkarcade,
  generators: [timestamp_type: :utc_datetime],
  tracker_api_url: System.get_env("TRACKER_API_URL")

host = System.get_env("PHX_HOST") || "localhost"

# Configures the endpoint
config :zkarcade, ZkArcadeWeb.Endpoint,
  url: [host: host],
  adapter: Bandit.PhoenixAdapter,
  render_errors: [
    formats: [html: ZkArcadeWeb.ErrorHTML, json: ZkArcadeWeb.ErrorJSON],
    root_layout: {ZkArcadeWeb.Layouts, :root}
  ],
  pubsub_server: ZkArcade.PubSub,
  live_view: [signing_salt: "XkOXIXZ0"]

# Configures the database
config :zkarcade,
  ecto_repos: [ZkArcade.Repo],
  env: Mix.env()

# Configure esbuild (the version is required)
config :esbuild,
  version: "0.17.11",
  zkarcade: [
    args:
      ~w(js/app.js --bundle --target=es2020 --outdir=../priv/static/assets --external:/fonts/* --external:/images/*),
    cd: Path.expand("../assets", __DIR__),
    env: %{"NODE_PATH" => Path.expand("../deps", __DIR__)}
  ]

# Configure tailwind (the version is required)
config :tailwind,
  version: "3.4.9",
  zkarcade: [
    args: ~w(
      --config=tailwind.config.js
      --input=css/app.css
      --output=../priv/static/assets/app.css
    ),
    cd: Path.expand("../assets", __DIR__)
  ]

config :tails, :color_classes, [
  "primary",
  "secondary",
  "accent",
  "destructive",
  "muted",
  "popover",
  "card",
  "border",
  "input",
  "foreground",
  "muted-foreground",
  "popover-foreground",
  "card-foreground",
  "primary-foreground",
  "secondary-foreground",
  "accent-foreground",
  "destructive-foreground",
  "ring",
  "radius",
  "tooltip"
]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Configures Ethers, to interact with Ethereum contracts
config :ethers,
  # Defaults to: Ethereumex.HttpClient
  rpc_client: Ethereumex.HttpClient,
  # Defaults to: ExKeccak
  keccak_module: ExKeccak,
  # Defaults to: Jason
  json_module: Jason,
  # Defaults to: ExSecp256k1
  secp256k1_module: ExSecp256k1,
  # Defaults to: nil, see Ethers.Signer for more info
  default_signer: nil,
  # Defaults to: []
  default_signer_opts: []

# Using Ethereumex, you can specify a default JSON-RPC server url here for all requests.
config :ethereumex,
  url: System.get_env("RPC_URL")

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{config_env()}.exs"
