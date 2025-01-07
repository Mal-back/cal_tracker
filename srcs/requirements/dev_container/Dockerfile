FROM rust:1.83

RUN mkdir -p /code /root/.config

RUN apt-get -y update && apt-get -y update && apt -y install git wget

RUN wget https://github.com/neovim/neovim/releases/download/v0.10.2/nvim-linux64.tar.gz && \
                                                    tar -xf nvim-linux64.tar.gz && \
                                                    mv /nvim-linux64/bin/nvim /usr/bin/nvim && \
                                                    mv /nvim-linux64/share/* /usr/local/share/ && \
                                                    rm -rf /nvim-linux64 /nvim-linux64.tar.gz

COPY ./nvim_config/nvim /root/.config/nvim


ENTRYPOINT ["sh", "-c", "sleep infinity"]
