[
    {
        "name"            : "${app_name}-app",
        "image"           : "${repo_url}",
        "cpu"             : ${fargate_cpu},
        "memory"          : ${fargate_memory},
        "networkMode"     : "awsvpc",
        "logConfiguration": {
        "logDriver"       : "awslogs",
        "options": {
                "awslogs-group"        : "/ecs/${app_name}-app",
                "awslogs-region"       : "${aws_region}",
                "awslogs-stream-prefix": "ecs"
                }
            },
            "portMappings": [
                {
                    "containerPort": ${app_port},
                    "hostPort"     : ${app_port}
                }
            ]
        "secrets": [
            {
                "name"     : "ROCKET_ENV",
                "valueFrom": "${ROCKET_ENV}"
            }
        ]
    }
]
