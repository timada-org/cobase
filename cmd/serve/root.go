package serve

import (
	"fmt"
	"os"

	"github.com/ilyakaznacheev/cleanenv"
	"github.com/spf13/cobra"

	"github.com/timada-org/cobase/internal/http"
)

type ServeConfig struct {
	Zone   string `yaml:"zone" env:"COBASE_ZONE"`
	Addr   string `yaml:"addr" env:"COBASE_ADDR"`
	Log    string `yaml:"log" env:"COBASE_LOG"`
	Dsn    string `yaml:"dsn" env:"COBASE_DSN"`
	Pulsar struct {
		Url       string `yaml:"url" env:"COBASE_PULSAR_URL"`
		Namespace string `yaml:"namespace" env:"COBASE_PULSAR_NAMESPACE"`
	} `yaml:"pulsar"`
	Pikav struct {
		Url       string `yaml:"url" env:"COBASE_PIKAV_URL"`
		Namespace string `yaml:"namespace" env:"COBASE_PIKAV_NAMESPACE"`
	} `yaml:"pikav"`
	Jwks struct {
		Url string `yaml:"url" env:"COBASE_JWKS_URL"`
	} `yaml:"jwks"`
}

func NewServeCmd() (cmd *cobra.Command) {
	var configPath string
	var staticPath string

	cmd = &cobra.Command{
		Use:   "serve",
		Short: "Run timada cobase server",
		Run: func(cmd *cobra.Command, args []string) {
			var cfg ServeConfig

			if err := cleanenv.ReadConfig(configPath, &cfg); err != nil {
				fmt.Fprintln(os.Stderr, err)
				os.Exit(1)
			}

			server := http.NewServer(http.ServerOptions{
				StaticPath: staticPath,
				Addr:       cfg.Addr,
			})

			server.Start()
		},
	}

	cmd.Flags().StringVarP(&staticPath, "static", "s", "", "static dir path")
	cmd.Flags().StringVarP(&configPath, "config", "c", "", "config file path")
	cmd.MarkFlagRequired("config")

	return cmd
}
