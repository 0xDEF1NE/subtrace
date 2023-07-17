from pydantic import BaseModel
from typing import Optional, Dict
from pathlib import Path
import os

class GlobalParams(BaseModel):
    version: bool = False
    url: Optional[str]
    list: Optional[str]
    templates: str = f"{Path.home()}/.config/subnerium/templates"
    timeout: int = 10
    debug: bool = False
    verbose: bool = False
    silent: bool = False
    output: str = None
    concurrency: int = 25
    local_home: str = str(Path.home())

verify_domains = []        

Gparams = GlobalParams()
