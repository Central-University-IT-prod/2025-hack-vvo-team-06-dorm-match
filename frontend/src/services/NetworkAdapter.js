class NetworkAdapter {
  static API_CONFIG = {
    AUTH: process.env.NODE_ENV === 'development' ? 'http://localhost:8080' : 'http://auth:8080',
    ROOM_MANAGEMENT: process.env.NODE_ENV === 'development' ? 'http://localhost:8081' : 'http://room-management:8081',
  };

  async request(method, service, endpoint, data = null, params = {}) {
    const baseUrl = NetworkAdapter.API_CONFIG[service];
    const endpointUrl = new URL(endpoint, baseUrl);
    const url = new URL(endpointUrl);

    Object.entries(params).forEach(([key, value]) => {
      url.searchParams.append(key, value);
    });

    const headers = {
      'Content-Type': 'application/json',
    };

    const token = localStorage.getItem('token');
    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }

    const options = {
      method,
      headers,
      credentials: 'include',
    };

    if (data) {
      options.body = JSON.stringify(data);
    }

    try {
      const response = await fetch(url.toString(), options);
      const json = await response.json();
      if (!response.ok) {
        return {
          data: {},
          errors: [json.errors?.[0] || `HTTP ${response.status}: ${response.statusText}`],
        };
      }
      return json;
    } catch (error) {
      return {
        data: {},
        errors: [error.message],
      };
    }
  }

  async login(email, password) {
    return this.request('POST', 'AUTH', '/auth/login', { email, password });
  }

  async register(userData) {
    return this.request('POST', 'AUTH', '/auth/register', userData);
  }

  async logout() {
    return this.request('POST', 'AUTH', '/auth/logout');
  }

  async getAuthUser() {
    return this.request('GET', 'AUTH', '/auth/user');
  }

  async updateProfile(profileData) {
    return this.request('PATCH', 'AUTH', '/auth/profile', profileData);
  }

  async createRoom(roomData) {
    return this.request('POST', 'ROOM_MANAGEMENT', '/rooms', roomData);
  }

  async searchRooms() {
    return this.request('GET', 'ROOM_MANAGEMENT', '/rooms/search');
  }

  async applyForRoom(userId, roomId) {
    return this.request('POST', 'ROOM_MANAGEMENT', '/rooms/apply', {
      user_id: userId,
      room_id: roomId,
    });
  }

  async getApplications(userId) {
    return this.request('GET', 'ROOM_MANAGEMENT', '/rooms/applications', { user_id: userId });
  }

  async approveApplication(applicationId, comment) {
    return this.request('POST', 'ROOM_MANAGEMENT', `/rooms/applications/${applicationId}/approve`, { comment });
  }

  async rejectApplication(applicationId, comment) {
    return this.request('POST', 'ROOM_MANAGEMENT', `/rooms/applications/${applicationId}/reject`, { comment });
  }

  async getRoomStats() {
    return this.request('GET', 'ROOM_MANAGEMENT', '/rooms/stats');
  }

  async autoAssignRoom(userId) {
    return this.request('POST', 'ROOM_MANAGEMENT', '/rooms/auto-assign', { user_id: userId });
  }
}

export const networkAdapter = new NetworkAdapter();
