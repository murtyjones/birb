[
    {
        "name"            : "birb-edgar-worker",
        "image"           : "${repo_url}",
        "cpu"             : ${cpu},
        "memory"          : ${memory},
        "logConfiguration": {
            "logDriver": "awslogs",
            "options"  : {
                "awslogs-group"        : "/ecs/birb-edgar-worker",
                "awslogs-region"       : "${aws_region}",
                "awslogs-stream-prefix": "ecs"
            }
        },
        "secrets": [
            {
                "name"     : "ROCKET_DATABASES",
                "valueFrom": "${ROCKET_DATABASES}"
            }
        ]
    }
]
