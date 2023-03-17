(import ./lib.nix) ({ self, pkgs, ... }: {
  name = "from-nixos";
  nodes = {
    # self here is set by using specialArgs in `lib.nix`
    node1 = { self, ... }: {
      imports = [ self.nixosModules.kld ];
      kuutamo.cockroachdb.nodeName = "kld-00";

      kuutamo.cockroachdb.caCertPath = ./cockroach-certs/ca.crt;
      kuutamo.cockroachdb.nodeCertPath = ./cockroach-certs + "/db1.crt";
      kuutamo.cockroachdb.nodeKeyPath = ./cockroach-certs + "/db1.key";
    };
  };

  extraPythonPackages = _p: [ self.packages.${pkgs.system}.remote-pdb ];

  # This test is still wip
  testScript = ''
    start_all()

    # wait for our service to start
    #node1.wait_for_unit("kld")

    # useful for debugging
    def remote_shell(machine):
        machine.shell_interact("tcp:127.0.0.1:4444,forever,interval=2")

    #remote_shell(machine)
    #breakpoint()
  '';
})
