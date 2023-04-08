# linux-fleet-manager
A tool to centrally manage a group of identical Debian or Debian-compatible servers, using SSH


### Project Goals

1 . Can manage the users, packages and system configuration of a fleet of identical Debian Linux hosts, using SSH
2. has a cool rusted metal theme with icons and artwork
3. configuration is done in a separate file, so that the main script does not need to be changed
4. configuration should include the target hosts with their related SSH configurations
5. configuration should include a global list of packages to install on all the hosts
6. configuration should include a list of users to create on all the hosts
7. configuration should include a list of systemd services to enable and/or restart
