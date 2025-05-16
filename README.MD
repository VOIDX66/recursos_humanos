# Recursos Humanos: _**My Humane**_  

[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/VOIDX66/recursos_humanos)

Sistema integral de gestión de recursos humanos.

## Descripción  
  
My Humane es una aplicación web para la gestión de recursos humanos que permite:  
- Gestión de usuarios con diferentes roles  
- Publicación y gestión de vacantes  
- Postulación a vacantes  
- Evaluación de candidatos  
- Seguimiento del proceso de contratación  
  
## Arquitectura  
  
El proyecto está dividido en dos componentes principales:  
  
### Backend  
- Desarrollado en **Rust** con Actix-web  
- API RESTful con autenticación JWT  
- Base de datos PostgreSQL  
  
### Frontend  
- Desarrollado con **Node.js** y Express.js  
- Sistema de plantillas EJS  
- Interfaz de usuario adaptada a diferentes roles  
  
## Requisitos previos  
  
- **Rust**
- **PostgreSQL**
- **Cargo**
- **Node.js**: Entorno de ejecución para JavaScript  
- **Express.js**: Framework web para Node.js  
- **EJS**: Sistema de plantillas para generar HTML  
- **JavaScript**: Para la lógica del cliente
  
## Instalación y configuración  
  
### 1. Clonar el repositorio  
```bash  
git clone https://github.com/VOIDX66/recursos_humanos.git  
cd recursos_humanos  