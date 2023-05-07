from typing import List, Optional, Dict
from pydantic import BaseModel

class Info(BaseModel):
    name: str
    author: str
    severity: str
    description: Optional[str] = None
    reference: List[str]
    tags: List[str]
    
class Settings(BaseModel):
    concatenate: Optional[bool] = False

class Matchers(BaseModel):
    type: Optional[str]
    part: Optional[str]
    words: Optional[List[str]] = None
    indice: Optional[int] = None
    status: Optional[List[int]] = None
    value: Optional[str] = None
    
class Keys(BaseModel):
    subdomains: List[str]

class Request(BaseModel):
    method: str
    path: str
    headers: Optional[Dict]
    matchers: List[Matchers] = None
    
    
class Templates(BaseModel):
    id: str
    info: Info
    requests: List[Request]
    settings: Optional[Settings] = False
