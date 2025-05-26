#!/usr/bin/env python3
import sys
import argparse
import os
from reportlab.lib.pagesizes import LETTER
from reportlab.lib.units import inch
from reportlab.platypus import SimpleDocTemplate, Paragraph, Spacer, Image, PageBreak
from reportlab.lib.styles import getSampleStyleSheet, ParagraphStyle
from reportlab.lib.enums import TA_JUSTIFY, TA_CENTER
from reportlab.pdfbase import pdfmetrics
from reportlab.pdfbase.ttfonts import TTFont
from reportlab.lib.colors import black

DEFAULT_FONT_PATH = "/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf"  # Ajusta según tu sistema
DEFAULT_FONT_NAME = "DejaVuSerif"
EMPRESA_NOMBRE = "My Humane"
POLITICAS_EMPRESA = ("Políticas de la empresa My Humane: "
                     "Cumplimiento normativo, confidencialidad, respeto y responsabilidad en el ambiente laboral. "
                     "Todos los empleados deben respetar el código de ética y las disposiciones internas.")

def cargar_fuente(fuente_path):
    if fuente_path and os.path.isfile(fuente_path):
        try:
            font_name = "CustomFont"
            pdfmetrics.registerFont(TTFont(font_name, fuente_path))
            return font_name
        except Exception as e:
            print(f"Advertencia: No se pudo cargar la fuente {fuente_path}. Usando fuente por defecto.")
    # Carga fuente por defecto
    if os.path.isfile(DEFAULT_FONT_PATH):
        try:
            pdfmetrics.registerFont(TTFont(DEFAULT_FONT_NAME, DEFAULT_FONT_PATH))
            return DEFAULT_FONT_NAME
        except:
            pass
    # Usa fuente builtin si no encuentra la fuente ttf
    return "Times-Roman"


def crear_contrato(titulo, salario, nombre_usuario, id_number, fuente_path, firma_path, salida):
    font_name = cargar_fuente(fuente_path)

    doc = SimpleDocTemplate(salida, pagesize=LETTER,
                            rightMargin=72, leftMargin=72,
                            topMargin=72, bottomMargin=72)

    styles = getSampleStyleSheet()
    # Redefinir estilo normal con nuestra fuente y justificado
    styles.add(ParagraphStyle(name='NormalJustify',
                              fontName=font_name,
                              fontSize=12,
                              leading=18,
                              alignment=TA_JUSTIFY,
                              spaceAfter=12))
    # Título centrado grande
    styles.add(ParagraphStyle(name='TitleCenter',
                              fontName=font_name,
                              fontSize=24,
                              alignment=TA_CENTER,
                              spaceAfter=24))

    # Subtítulos en negrita
    styles.add(ParagraphStyle(name='SubTitleBold',
                              fontName=font_name,
                              fontSize=12,
                              leading=18,
                              alignment=TA_JUSTIFY,
                              spaceBefore=12,
                              spaceAfter=6,
                              textColor=black,
                              fontWeight='bold'))

    # Para el texto en negrita normal
    styles.add(ParagraphStyle(name='BoldText',
                              parent=styles['NormalJustify'],
                              fontName=font_name,
                              fontSize=12,
                              leading=18,
                              alignment=TA_JUSTIFY,
                              spaceAfter=6,
                              textColor=black,
                              ))

    story = []

    # Título empresa
    story.append(Paragraph(EMPRESA_NOMBRE, styles['TitleCenter']))

    # Texto formal contrato
    texto = f"""
    <b>Entre las partes:</b><br/>
    Por un lado, <b>{EMPRESA_NOMBRE}</b>, identificado con NIT No. 123456789-0, en adelante "EL CONTRATANTE".<br/>
    Y por otro, <b>{nombre_usuario}</b>, identificado con cédula No. {id_number}, en adelante "EL CONTRATADO".<br/><br/>

    <b>PRIMERO. OBJETO DEL CONTRATO:</b><br/>
    El presente contrato tiene por objeto la vacante titulada <b>{titulo}</b> {f"con un salario de ${salario} COP" if salario else "sin especificar salario"}.<br/><br/>

    <b>SEGUNDO. OBLIGACIONES DE LAS PARTES:</b><br/>
    EL CONTRATANTE se obliga a proveer los medios necesarios para el adecuado desempeño de las funciones asignadas.<br/>
    EL CONTRATADO se obliga a cumplir con las responsabilidades y tareas asignadas conforme a la vacante.<br/><br/>

    <b>TERCERO. PLAZOS:</b><br/>
    Las obligaciones comenzarán a partir de la firma de este contrato y tendrán la duración estipulada por la legislación vigente.<br/><br/>

    <b>CUARTO. PRECIO Y FORMA DE PAGO:</b><br/>
    {f"El salario pactado es de ${salario} COP, pagadero conforme a la política de la empresa." if salario else "El salario será acordado entre las partes y consignado en documento aparte."}<br/><br/>

    <b>QUINTO. CLAUSULAS ESPECIALES:</b><br/>
    {POLITICAS_EMPRESA}<br/><br/>

    <b>SEXTO. RESOLUCIÓN DEL CONTRATO:</b><br/>
    El contrato podrá ser terminado de conformidad con las causas establecidas en la legislación laboral vigente.<br/><br/>

    <b>SÉPTIMO. LUGAR DE RESIDENCIA Y NOTIFICACIONES:</b><br/>
    Las partes señalan como domicilios para notificaciones los indicados en sus documentos de identificación.<br/><br/>

    <b>OCTAVO. FUERO JURISDICCIONAL:</b><br/>
    Para todas las controversias derivadas de este contrato, las partes se someten a la jurisdicción de los tribunales competentes de la ciudad donde se firma el contrato.<br/><br/>

    <b>NOVENO. FIRMA:</b><br/>
    En señal de conformidad, las partes firman el presente contrato en dos ejemplares, en la ciudad de ___________, a los ____ días del mes de ___________ del año ______.<br/><br/>
    """

    story.append(Paragraph(texto, styles['NormalJustify']))
    story.append(PageBreak())

    # Página de firmas

    story.append(Paragraph(f"{EMPRESA_NOMBRE}", styles['TitleCenter']))
    story.append(Paragraph("Representante Legal", styles['SubTitleBold']))

    # Firma representante
    if firma_path and os.path.isfile(firma_path):
        try:
            img = Image(firma_path, width=2*inch, height=1*inch)
            img.hAlign = 'LEFT'
            story.append(img)
        except Exception as e:
            story.append(Paragraph("(Firma no disponible)", styles['NormalJustify']))
    else:
        story.append(Paragraph("(Firma no disponible)", styles['NormalJustify']))

    story.append(Spacer(1, 60))

    # Firma contratado
    story.append(Paragraph(f"{nombre_usuario}", styles['TitleCenter']))
    story.append(Paragraph("EL CONTRATADO", styles['SubTitleBold']))
    story.append(Spacer(1, 80))
    story.append(Paragraph("(Firma del Contratado)", styles['NormalJustify']))

    doc.build(story)

def main():
    parser = argparse.ArgumentParser(description="Generar contrato PDF formal.")
    parser.add_argument("--titulo", required=True, help="Título de la vacante")
    parser.add_argument("--salario", default=None, help="Salario (opcional)")
    parser.add_argument("--nombre", required=True, help="Nombre completo del usuario")
    parser.add_argument("--id", required=True, help="Documento de identificación del usuario")
    parser.add_argument("--fuente", default=None, help="Ruta al archivo .ttf de la fuente")
    parser.add_argument("--firma", default=None, help="Ruta a la imagen de la firma del representante")
    parser.add_argument("--salida", required=True, help="Ruta donde guardar el PDF generado")

    args = parser.parse_args()

    crear_contrato(args.titulo, args.salario, args.nombre, args.id, args.fuente, args.firma, args.salida)


if __name__ == "__main__":
    main()
