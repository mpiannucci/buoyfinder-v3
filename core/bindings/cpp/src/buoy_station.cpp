#include "buoyfinder/buoy_station.hpp"
#include "buoyfinder.h"

namespace buoyfinder {

    BFBuoyStation::BFBuoyStation(BuoyStation* station) :
        m_ptr(station) {
    }

    BFBuoyStation::BFBuoyStation(const char* station_id, const char* name, double lat, double lon) :
        m_ptr(nullptr) {

		m_ptr = buoy_station_new(station_id, name, lat, lon);
    }

    BFBuoyStation::~BFBuoyStation() {
		buoy_station_free(m_ptr);
		m_ptr = nullptr;
    }

    const std::string BFBuoyStation::stationId() const {
		if (m_ptr == nullptr) {
			return "";
		}
		return buoy_station_id(m_ptr);
    }

    const std::string BFBuoyStation::name() const {
		if (m_ptr == nullptr) {
			return "";
		}
		return buoy_station_name(m_ptr);
    }

    const bool BFBuoyStation::isActive() const {
		if (m_ptr == nullptr) {
			return false;
		}
		return buoy_station_active(m_ptr);
    }
}