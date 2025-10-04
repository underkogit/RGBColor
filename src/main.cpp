#include <Arduino.h>

typedef struct {
  int num1;
  int num2;
  int num3;
  int num4;
  int num5;
} RGBTuple;

void setup() {
  // Initialize serial communication at 9600 bits per second:
  Serial.begin(9600);
}

void loop() {
  if (Serial.available()) {
    String data = Serial.readStringUntil('\n');
    
    // Удаляем лишние символы и форматируем строку
    String cleanedData = data.substring(0, data.indexOf(';'));

    RGBTuple rgbTuple;
    int count = sscanf(cleanedData.c_str(), "%d,%d,%d,%d,%d", &rgbTuple.num1, &rgbTuple.num2, &rgbTuple.num3, &rgbTuple.num4, &rgbTuple.num5);
    
    if (count == 5) {
      Serial.print("Получено от компьютера: ");
      for (int i = 0; i < count; i++) {
        Serial.print(rgbTuple.num1);
        if (i < count - 1) {
          Serial.print(", ");
        }
      }
      Serial.println();
      
      // Отправьте ответное сообщение назад в компьютер
      Serial.println("Ответное сообщение от устройства");
    } else {
      Serial.println("Ошибка: Получены некорректные данные.");
    }
  }
}
