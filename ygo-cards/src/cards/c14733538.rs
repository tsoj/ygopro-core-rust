//! Draco Face-Off
use super::prelude::*;

pub const ID: u32 = 14733538;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_DRACOSLAYER, SET_DRACOVERLORD], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetProperty(EFFECT_FLAG_LIMIT_ZONE)
    effect::set_property(d, e1, EFFECT_FLAG_LIMIT_ZONE, 0);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.target)
    effect::set_target(d, e1, Some(cost_target(target)));
    // e1:SetOperation(s.activate)
    effect::set_operation(d, e1, Some(operation(activate)));
    // e1:SetValue(s.zones)
    effect::set_value_fn(d, e1, lua_fn(zones));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.filter(c,e,tp,b1,setcode)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, b1: bool, setcode: u32) -> bool {
    // return c:IsSetCard(setcode) and c:IsType(TYPE_PENDULUM) and not c:IsForbidden()
    //     and (b1 or c:IsCanBeSpecialSummoned(e,0,tp,false,false))
    card::is_set_card(d, c, &[setcode as u16])
        && card::is_type(d, c, TYPE_PENDULUM)
        && !card::is_forbidden(d, c)
        && (b1 || card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None))
}
/// `s.zones(e,tp,eg,ep,ev,re,r,rp)`
fn zones(d: &mut Duel, a: Args) -> i64 {
    let e = a.e(0);
    let tp = a.p(1);
    // local zone=0xff --all Spell/Trap zones
    let mut zone: i64 = 0xff;
    // local p0=Duel.CheckLocation(tp,LOCATION_PZONE,0)
    let p0 = duel::check_location(d, tp, LOCATION_PZONE, 0);
    // local p1=Duel.CheckLocation(tp,LOCATION_PZONE,1)
    let p1 = duel::check_location(d, tp, LOCATION_PZONE, 1);
    // local sp=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,false,SET_DRACOSLAYER)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,false,SET_DRACOVERLORD)
    let sp = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
        && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp, false, SET_DRACOSLAYER)), tp, LOCATION_DECK, 0, 1, NO_EX)
        && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, e, tp, false, SET_DRACOVERLORD)), tp, LOCATION_DECK, 0, 1, NO_EX);
    // if p0==p1 or sp then return zone end
    if p0 == p1 || sp {
        return zone;
    }
    // if p0 then zone=zone-0x1 end --remove the left most S/T zone
    if p0 {
        zone -= 0x1;
    }
    // if p1 then zone=zone-0x10 end --remove the right most S/T zone
    if p1 {
        zone -= 0x10;
    }
    // return zone
    zone
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local b1=Duel.CheckPendulumZones(tp)
    let b1 = duel::check_pendulum_zones(d, tp);
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let b2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // if chk==0 then return (b1 or b2)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,b1,SET_DRACOSLAYER)
    //     and Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil,e,tp,b1,SET_DRACOVERLORD) end
    if a.chk == Some(0) {
        return (b1 || b2)
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, tp, b1, SET_DRACOSLAYER)), tp, LOCATION_DECK, 0, 1, NO_EX)
            && duel::is_existing_matching_card(d, Some(&|d, c| filter(d, c, a.e, tp, b1, SET_DRACOVERLORD)), tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,nil,0,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, None, 0, tp, LOCATION_DECK as i32);
    true
}
/// `s.activate(e,tp,eg,ep,ev,re,r,rp)`
fn activate(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    let e = a.e;
    // local b1=Duel.CheckPendulumZones(tp)
    let b1 = duel::check_pendulum_zones(d, tp);
    // local b2=Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    let b2 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0;
    // if not b1 and not b2 then return end
    if !b1 && !b2 {
        return;
    }
    // local g1=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil,e,tp,b1,SET_DRACOSLAYER)
    let g1 = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e, tp, b1, SET_DRACOSLAYER)), tp, LOCATION_DECK, 0, NO_EX);
    // local g2=Duel.GetMatchingGroup(s.filter,tp,LOCATION_DECK,0,nil,e,tp,b1,SET_DRACOVERLORD)
    let g2 = duel::get_matching_group(d, Some(&|d, c| filter(d, c, e, tp, b1, SET_DRACOVERLORD)), tp, LOCATION_DECK, 0, NO_EX);
    // if #g1==0 or #g2==0 then return end
    if g1.is_empty() || g2.is_empty() {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local sg1=g1:Select(tp,1,1,nil)
    let mut sg1 = group::select(d, &g1, tp, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_CONFIRM)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_CONFIRM as u64);
    // local sg2=g2:Select(tp,1,1,nil)
    let sg2 = group::select(d, &g2, tp, 1, 1, false, NO_EX).nil();
    // sg1:Merge(sg2)
    group::add_card(&mut sg1, &sg2);
    // Duel.ConfirmCards(1-tp,sg1)
    duel::confirm_cards(d, 1 - tp, &sg1);
    // Duel.ShuffleDeck(tp)
    duel::shuffle_deck(d, tp);
    // local cg=sg1:Select(1-tp,1,1,nil)
    let cg = group::select(d, &sg1, 1 - tp, 1, 1, false, NO_EX).nil();
    // local tc=cg:GetFirst()
    let tc = group::get_first(d, &cg).nil();
    // Duel.Hint(HINT_CARD,0,tc:GetCode())
    let code = card::get_code(d, tc)[0];
    duel::hint(d, HINT_CARD as u8, 0, code as u64);
    // if b1 and (not b2 or not tc:IsCanBeSpecialSummoned(e,0,tp,false,false) or Duel.SelectYesNo(tp,aux.Stringid(id,0))) then
    let to_pzone = b1
        && (!b2
            || !card::is_can_be_special_summoned(d, tc, e, 0, tp, false, false, None, None, None)
            || duel::select_yes_no(d, tp, aux::stringid(ID, 0)));
    if to_pzone {
        // Duel.MoveToField(tc,tp,tp,LOCATION_PZONE,POS_FACEUP,true)
        duel::move_to_field(d, tc, tp, tp, LOCATION_PZONE as u16, POS_FACEUP as u8, true, None);
    } else {
        // Duel.SpecialSummon(tc,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, tc, 0, tp, tp, false, false, POS_FACEUP as u8, None);
    }
    // sg1:RemoveCard(tc)
    group::remove_card(&mut sg1, &CardSet::from(tc));
    // Duel.SendtoExtraP(sg1,nil,REASON_EFFECT)
    duel::send_to_extra_p(d, sg1, None, REASON_EFFECT, None);
}
