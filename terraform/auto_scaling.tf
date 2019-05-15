resource "aws_appautoscaling_target" "api_autoscaling_target" {
  service_namespace  = "ecs"
  resource_id        = "service/${aws_ecs_cluster.api_cluster.name}/${aws_ecs_service.api_service.name}"
  scalable_dimension = "ecs:service:DesiredCount"
  role_arn           = "${aws_iam_role.autoscale_role.arn}"
  min_capacity       = 1
  max_capacity       = 6
}

# Automatically scale capacity up by one
resource "aws_appautoscaling_policy" "api_scale_up" {
  name               = "birb_api_scale_up"
  service_namespace  = "ecs"
  resource_id        = "service/${aws_ecs_cluster.api_cluster.name}/${aws_ecs_service.api_service.name}"
  scalable_dimension = "ecs:service:DesiredCount"

  step_scaling_policy_configuration {
    adjustment_type         = "ChangeInCapacity"
    cooldown                = 60
    metric_aggregation_type = "Maximum"

    step_adjustment {
      metric_interval_lower_bound = 0
      scaling_adjustment          = 1
    }
  }

  depends_on = ["aws_appautoscaling_target.api_autoscaling_target"]
}

# Automatically scale capacity down by one
resource "aws_appautoscaling_policy" "api_scale_down" {
  name               = "birb_api_scale_down"
  service_namespace  = "ecs"
  resource_id        = "service/${aws_ecs_cluster.api_cluster.name}/${aws_ecs_service.api_service.name}"
  scalable_dimension = "ecs:service:DesiredCount"

  step_scaling_policy_configuration {
    adjustment_type         = "ChangeInCapacity"
    cooldown                = 60
    metric_aggregation_type = "Maximum"

    step_adjustment {
      metric_interval_upper_bound = 0
      scaling_adjustment          = -1
    }
  }

  depends_on = ["aws_appautoscaling_target.api_autoscaling_target"]
}

# Cloudwatch alarm that triggers the autoscaling up policy
resource "aws_cloudwatch_metric_alarm" "api_cpu_utilization_high" {
  alarm_name          = "birb_api_cpu_utilization_high"
  comparison_operator = "GreaterThanOrEqualToThreshold"
  evaluation_periods  = "2"
  metric_name         = "CPUUtilization"
  namespace           = "AWS/ECS"
  period              = "60"
  statistic           = "Average"
  threshold           = "85"

  dimensions {
    ClusterName = "${aws_ecs_cluster.api_cluster.name}"
    ServiceName = "${aws_ecs_service.api_service.name}"
  }

  alarm_actions = ["${aws_appautoscaling_policy.api_scale_up.arn}"]
}

# Cloudwatch alarm that triggers the autoscaling down policy
resource "aws_cloudwatch_metric_alarm" "api_cpu_utilization_low" {
  alarm_name          = "birb_api_cpu_utilization_low"
  comparison_operator = "LessThanOrEqualToThreshold"
  evaluation_periods  = "2"
  metric_name         = "CPUUtilization"
  namespace           = "AWS/ECS"
  period              = "60"
  statistic           = "Average"
  threshold           = "10"

  dimensions {
    ClusterName = "${aws_ecs_cluster.api_cluster.name}"
    ServiceName = "${aws_ecs_service.api_service.name}"
  }

  alarm_actions = ["${aws_appautoscaling_policy.api_scale_down.arn}"]
}
