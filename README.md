# GitLab note MR

Small service to add notes to GitLab MRs from CI pipelines, while access tokens are not exposed to those pipelines. The service can be set up to use different API tokens on the same gitlab instance.

The CI tool can use `curl` or other methods to dial out to this service, which authenticates with HTTP Basic Auth. A JSON struct is accepted as input for for note body, and it returns with appropriate status codes which can be handled by `curl` itself.

## Configure

The application reads a configuration file provided as a parameter of the executable with the following structure:

```yaml
---
bind: "0.0.0.0:8080" # port the web service should bind to; defaults to "0.0.0.0:8080"
gitlab_url: "https://gitlab.example.com" # GitLab server external URL stub; defaults to "https://gitlab.com".
log:
  level: info # log level; can be one of "off", "error", "warn", "info", "debug", "trace"
targets:
- user: username # request username
  pass: password # request password
  token: glplat-XXXXXXXXX # gitlab API token
```

## Run

The CLI takes a single argument, the config file's location. Example:

```shell
$ cargo build -r && ./target/release/gitlab_note_mr config.yml
    Finished release [optimized] target(s) in 0.10s
INFO - listening on 0.0.0.0:8080
...
```

Then, a client can send a note to a project's MR using convenient BaseAuth, like it would using the potentially dangerous token:

```shell
$ curl -v -H Content-Type:application/json --data '{"body":"message from gitlab-note-mr app"}' http://username:password@localhost:8080/note/12345/1
*   Trying 127.0.0.1:8080...
* Connected to localhost (127.0.0.1) port 8080 (#0)
* Server auth using Basic with user 'username'
> POST /note/1307/1 HTTP/1.1
> Host: localhost:8080
> Authorization: Basic dXNlcm5hbWU6cGFzc3dvcmQ=
> User-Agent: curl/7.79.1
> Accept: */*
> Content-Type:application/json
> Content-Length: 47
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 201 Created
< content-length: 0
< date: Wed, 26 Apr 2023 10:07:08 GMT
<
* Connection #0 to host localhost left intact
```

## Licensing

SPDX-License-Identifier: BlueOak-1.0.0 OR MIT

This software is licensed under two licenses of your choice: Blue Oak Public License 1.0, or MIT Public License.
