FROM ubuntu:24.04 AS prod

RUN apt-get update && \
    apt-get install -y curl ca-certificates tzdata git gcc build-essential

ARG PYTHON_VERSION=3.12.4
ARG PROJECT_NAME=vector-serve

ARG UV_PYTHON_INSTALL_DIR=/opt/python

ENV UV_PYTHON_INSTALL_DIR=${UV_PYTHON_INSTALL_DIR}
ENV PROJECT_HOME=/opt/${PROJECT_NAME}

ENV VIRTUAL_ENV=${PROJECT_HOME}/.venv
ENV PATH="${VIRTUAL_ENV}/bin:$PATH"

WORKDIR ${PROJECT_HOME}

COPY --from=ghcr.io/astral-sh/uv:latest /uv /bin/uv

ENV UV_LINK_MODE=copy
RUN uv venv --python "${PYTHON_VERSION}"

COPY vector-serve .

RUN uv sync --all-groups

RUN uv run python -m app.models

CMD ["uv", "run", "fastapi", "run", "app/app.py", "--host", "0.0.0.0", "--port", "3000", "--workers", "1"]
