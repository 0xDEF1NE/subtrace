from pydantic import BaseModel
from typing import Optional, Dict

class GlobalParams(BaseModel):
    update: bool = False
    version: bool = False
    url: Optional[str]
    templates: str = "/usr/share/subnerium/nerium-templates"
    timeout: int = 10
    debug: bool = False
    verbose: bool = False
    silent: bool = False
    output: str = None
    concurrency: int = 25

verify_domains = []        

Gparams = GlobalParams()
