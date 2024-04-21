FROM purtontech/rust-on-nails-devcontainer:1.2.1 AS development
# this pulls down the config and installs the dependencies
# https://hub.docker.com/layers/purtontech/rust-on-nails-devcontainer/1.2.1/images/sha256-595ad34c9bfb3971daa617304dd6880de3603a60284406055cb22e5e602369d8?context=explore

COPY ps1.bash .
RUN cat ps1.bash >> ~/.bashrc && sudo rm ps1.bash

COPY .bash_aliases /home/vscode/.bash_aliases

# Enable our git hooks and set the permisisons on docker sock.
RUN echo 'git config core.hooksPath /workspace/.devcontainer/.githooks' >> ~/.bashrc

# all the volumes configured in the docker-compose.yml
ARG USERNAME=vscode
RUN sudo mkdir -p /workspace/target && sudo chown $USERNAME:$USERNAME /workspace/target
#RUN sudo mkdir -p /workspace/crates/asset-pipeline/node_modules && sudo chown $USERNAME:$USERNAME /workspace/crates/asset-pipeline/node_modules