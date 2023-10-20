from typing import List, Optional, Dict
from pydantic import BaseModel

class Info(BaseModel):
    name: str = None
    author: str = None
    severity: str = None
    description: Optional[str] = None
    reference: List[str] = None
    tags: List[str] = None
    
class Settings(BaseModel):
    concatenate: Optional[bool] = False

class Matchers(BaseModel):
    type: Optional[str] = None
    part: Optional[str] = None
    words: Optional[List[str]] = None
    indice: Optional[int] = None
    status: Optional[List[int]] = None
    value: Optional[str] = None
    
class Keys(BaseModel):
    subdomains: List[str] = None

class Request(BaseModel):
    method: str = None
    path: str = None
    headers: Optional[Dict] = None
    matchers: List[Matchers] = None
    
    
class Templates(BaseModel):
    id: str = None
    info: Info = None
    requests: List[Request] = None
    settings: Optional[Settings] = False
