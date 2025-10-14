import os
import json
import re
import google.generativeai as genai
from flask import Flask, request, jsonify
from dotenv import load_dotenv
from pydantic import BaseModel
from typing import List

load_dotenv()
app = Flask(__name__)

# --- 1. กำหนด Data Contracts ด้วย Pydantic ---
class AIAnalysisRequest(BaseModel):
    user_id: str
    posts: List[str]

class AIAnalysisResponse(BaseModel):
    personality_tags: List[str]
    suggested_theme: str

# --- 2. ตั้งค่า Gemini API ---
genai.configure(api_key=os.getenv("GOOGLE_API_KEY"))
model = genai.GenerativeModel('gemini-2.5-pro')  # ใช้รุ่นเสถียรสุดตอนนี้

# --- 3. Endpoint: วิเคราะห์บุคลิกภาพ ---
@app.route('/analyze-personality', methods=['POST'])
def analyze_personality_endpoint():
    try:
        request_data = AIAnalysisRequest(**request.get_json())
    except Exception as e:
        return jsonify({'error': 'Invalid request body', 'details': str(e)}), 400

    try:
        all_posts = "\n".join(request_data.posts)
        prompt = f"""
You are an AI personality analyzer.

Analyze the following social media posts and describe the user's personality in 3–5 short keywords
that represent their style or vibe (e.g., creative, formal, minimalist, tech-savvy).
Then, suggest exactly one theme name suitable for their web design (e.g., "dark_minimalist", "playful_vibrant").

Posts:
{all_posts}

Respond ONLY with a single valid JSON object — no extra text, no explanation, no markdown code block.
Use this exact structure:

{{
  "personality_tags": ["string", "string", "string"],
  "suggested_theme": "string"
}}
"""

        # --- เรียกใช้งาน Gemini ---
        response = model.generate_content(
            prompt,
            generation_config={"response_mime_type": "application/json"}  # บังคับให้เป็น JSON
        )

        clean_text = response.text.strip()
        if not clean_text.startswith("{"):
            match = re.search(r"\{.*\}", clean_text, re.DOTALL)
            clean_text = match.group(0) if match else clean_text

        result = json.loads(clean_text)
        validated_response = AIAnalysisResponse(**result)

        return jsonify(validated_response.dict())

    except Exception as e:
        print(f"Error calling Gemini or parsing response: {e}")
        return jsonify({'error': 'Failed to process AI request', 'details': str(e)}), 500


# --- 4. Endpoint: แชตปกติ (ถาม-ตอบทั่วไป) ---
@app.route('/chat', methods=['POST'])
def chat_endpoint():
    try:
        data = request.get_json()
        user_message = data.get("message", "").strip()

        if not user_message:
            return jsonify({"error": "Missing 'message' field"}), 400

        # 🟢 เพิ่ม system prompt ที่กำหนดบทบาทของ AI
        system_prompt = (
            "คุณคือ LivingProfile AI — "
            "ผู้ช่วยอัจฉริยะที่ช่วยผู้ใช้สร้างโปรไฟล์ส่วนตัว "
            "โดยเข้าใจบุคลิก นิสัย ความสนใจ และสไตล์ของพวกเขา "
            "พูดจาเป็นมิตร ฉลาด และอบอุ่น\n\n"
            "ตอนนี้ผู้ใช้กำลังคุยกับคุณ:\n"
        )

        # รวม prompt + ข้อความของผู้ใช้
        full_prompt = system_prompt + user_message

        response = model.generate_content(full_prompt)
        return jsonify({"reply": response.text.strip()})

    except Exception as e:
        print(f"Chat error: {e}")
        return jsonify({"error": "Failed to process chat", "details": str(e)}), 500

# --- 5. แสดงโมเดลที่ใช้งานได้ (debug) ---
if __name__ == '__main__':
    for m in genai.list_models():
        if 'generateContent' in m.supported_generation_methods:
            print(m.name)

    app.run(host='0.0.0.0', port=8001, debug=True)
