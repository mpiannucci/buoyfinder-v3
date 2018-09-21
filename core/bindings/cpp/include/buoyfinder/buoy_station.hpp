#pragma once

#include <string>

struct BuoyStation;

namespace buoyfinder
{
    class BFBuoyStation {

        public:
            BFBuoyStation(BuoyStation* station);
            BFBuoyStation(const char* station_id, const char* name, double lat, double lon);
            virtual ~BFBuoyStation();

            const std::string stationId() const;
            const std::string name() const;
            const bool isActive() const;

        private:
            BuoyStation* m_ptr;
    };
} // buoyfinder
