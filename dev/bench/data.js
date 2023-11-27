window.BENCHMARK_DATA = {
  "lastUpdate": 1701047334007,
  "repoUrl": "https://github.com/Javagedes/edk2_parser",
  "entries": {
    "Rust Benchmark": [
      {
        "commit": {
          "author": {
            "name": "Javagedes",
            "username": "Javagedes"
          },
          "committer": {
            "name": "Javagedes",
            "username": "Javagedes"
          },
          "id": "29f9daea0b9e27c2a2a2fe4ab910c29267aa7941",
          "message": "add benchmarking to github actions",
          "timestamp": "2023-11-25T20:56:46Z",
          "url": "https://github.com/Javagedes/edk2_parser/pull/3/commits/29f9daea0b9e27c2a2a2fe4ab910c29267aa7941"
        },
        "date": 1701046583654,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse_inf/baseLib",
            "value": 1916863,
            "range": "± 83582",
            "unit": "ns/iter"
          },
          {
            "name": "parse_inf/opensslLib",
            "value": 2769841,
            "range": "± 45982",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "name": "Javagedes",
            "username": "Javagedes"
          },
          "committer": {
            "name": "Javagedes",
            "username": "Javagedes"
          },
          "id": "29f9daea0b9e27c2a2a2fe4ab910c29267aa7941",
          "message": "add benchmarking to github actions",
          "timestamp": "2023-11-25T20:56:46Z",
          "url": "https://github.com/Javagedes/edk2_parser/pull/3/commits/29f9daea0b9e27c2a2a2fe4ab910c29267aa7941"
        },
        "date": 1701046583654,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse_inf/baseLib",
            "value": 1916863,
            "range": "± 83582",
            "unit": "ns/iter"
          },
          {
            "name": "parse_inf/opensslLib",
            "value": 2769841,
            "range": "± 45982",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "joey.vagedes@gmail.com",
            "name": "Joey Vagedes",
            "username": "Javagedes"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "e839428c77dec2d689a0f25611e16ab461b11e06",
          "message": "add benchmarking to github actions (#3)\n\nAdds the first benchmark, benchmark_inf_parse.rs to the codebase and\r\ncreates a github action to run benchmarks on every pull request and\r\npush to the main branch utilzing the criterion crate and the\r\ngithub-action-benchmark action.",
          "timestamp": "2023-11-26T17:07:31-08:00",
          "tree_id": "3e0144228261cec32eb9f5d8d93faa1c18b4aab8",
          "url": "https://github.com/Javagedes/edk2_parser/commit/e839428c77dec2d689a0f25611e16ab461b11e06"
        },
        "date": 1701047333635,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse_inf/baseLib",
            "value": 1912395,
            "range": "± 13371",
            "unit": "ns/iter"
          },
          {
            "name": "parse_inf/opensslLib",
            "value": 2771051,
            "range": "± 28789",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}