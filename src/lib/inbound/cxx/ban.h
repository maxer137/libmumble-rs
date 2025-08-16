
struct Ban {
	bool isExpired() const;
	bool isValid() const;
	bool operator<(const Ban &) const;
	bool operator==(const Ban &) const;
};

