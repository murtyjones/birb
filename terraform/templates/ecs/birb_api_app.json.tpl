[
    {
        "name"            : "birb-api",
        "image"           : "${repo_url}",
        "cpu"             : ${birb_api_cpu},
        "memory"          : ${birb_api_memory},
        "networkMode"     : "awsvpc",
        "logConfiguration": {
            "logDriver": "awslogs",
            "options"  : {
                "awslogs-group"        : "/ecs/birb-api",
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
