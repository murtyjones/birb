[
    {
        "name"            : "birb-api-app",
        "image"           : "${repo_url}",
        "cpu"             : ${fargate_cpu},
        "memory"          : ${fargate_memory},
        "networkMode"     : "awsvpc",
        "logConfiguration": {
            "logDriver": "awslogs",
            "options"  : {
                "awslogs-group"        : "/ecs/birb-api-app",
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
