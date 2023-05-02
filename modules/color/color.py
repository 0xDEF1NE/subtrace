from colorama import Fore, Style

def get_color_logs(log):
    fg_color = ''
    if log == "info":
        fg_color = Fore.BLUE
    elif log == "success":
        fg_color = Fore.GREEN
    elif log == "warning":
        fg_color = Fore.YELLOW
    elif log == "error":
        fg_color = Fore.RED  # cor magenta ao invés de laranja
    return f"{fg_color}[{log}]{Style.RESET_ALL}"

def get_color_severity(severity):
    fg_color = ''
    if severity == "info":
        fg_color = Fore.BLUE
    elif severity == "low":
        fg_color = Fore.GREEN
    elif severity == "medium":
        fg_color = Fore.YELLOW
    elif severity == "high":
        fg_color = Fore.RED  # cor magenta ao invés de laranja
    elif severity == "critical":
        fg_color = Fore.YELLOW
    return f"{fg_color}[{severity}]{Style.RESET_ALL}"

def colorize_logs(log):
    return get_color_logs(log)

def colorize_severity(severity):
    return get_color_severity(severity)
