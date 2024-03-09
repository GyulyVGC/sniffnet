FROM homebrew/ubuntu22.04
RUN brew install sniffnet
RUN ["sniffnet"]