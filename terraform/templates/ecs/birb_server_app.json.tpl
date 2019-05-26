[
    {
        "name"            : "birb-server",
        "image"           : "${repo_url}",
        "cpu"             : ${birb_server_cpu},
        "memory"          : ${birb_server_memory},
        "networkMode"     : "awsvpc",
        "logConfiguration": {
            "logDriver": "awslogs",
            "options"  : {
                "awslogs-group"        : "/ecs/birb-server",
                "awslogs-region"       : "${aws_region}",
                "awslogs-stream-prefix": "ecs"
            }
        },
        "portMappings": [
            {
                "containerPort": ${app_port},
                "hostPort"     : ${app_port}
            }
        ],
        "secrets": [
            {
                "name"     : "ROCKET_DATABASES",
                "valueFrom": "${ROCKET_DATABASES}"
            }
        ]
    }
]
