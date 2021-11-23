/**
 * @file dht22_sensor_read_temp.ino
 * @brief Based on the example from Grove_Temperature_And_Humidity_Sensor Arduino library:
 *      https://github.com/Seeed-Studio/Grove_Temperature_And_Humidity_Sensor/blob/master/examples/DHTtester/DHTtester.ino
 */

#include "DHT.h"

#define DHTTYPE DHT22   // DHT 22  (AM2302)

#define DHTPIN 7
DHT dht(DHTPIN, DHTTYPE);

#if defined(ARDUINO_ARCH_AVR)
    #define debug  Serial

#elif defined(ARDUINO_ARCH_SAMD) ||  defined(ARDUINO_ARCH_SAM)
    #define debug  SerialUSB
#else
    #define debug  Serial
#endif

constexpr const unsigned long SERIAL_BAUD_RATE = 115200;
constexpr const unsigned long MEASUREMENT_DELAY_MS = 2000;

void setup() {
    debug.begin(SERIAL_BAUD_RATE);
    dht.begin();
}

// Reading temperature or humidity takes about 250 milliseconds!
// Sensor readings may also be up to 2 seconds 'old' (its a very slow sensor)
void loop() {
    float temp_hum_val[2] = {0};
    if (!dht.readTempAndHumidity(temp_hum_val)) {
        debug.print("Humidity: ");
        debug.print(temp_hum_val[0]);
        debug.print(" %\t");
        debug.print("Temperature: ");
        debug.print(temp_hum_val[1]);
        debug.println(" *C");
    } else {
        debug.println("Failed to get temprature and humidity value.");
    }

    delay(MEASUREMENT_DELAY_MS);
}