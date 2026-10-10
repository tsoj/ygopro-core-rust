//! Buster Blader, the Destruction Swordmaster
use super::prelude::*;

pub const ID: u32 = 3428069;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[CARD_BUSTER_BLADER], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Code
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e1, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e1:SetCode(EFFECT_CHANGE_CODE)
    effect::set_code(d, e1, EFFECT_CHANGE_CODE);
    // e1:SetRange(LOCATION_MZONE|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_MZONE | LOCATION_GRAVE);
    // e1:SetValue(CARD_BUSTER_BLADER)
    effect::set_value(d, e1, CARD_BUSTER_BLADER as i64);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Equip
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e2, aux::stringid(ID, 0));
    // e2:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e2, CATEGORY_EQUIP);
    // e2:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e2:SetCode(EVENT_DESTROYED)
    effect::set_code(d, e2, EVENT_DESTROYED);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET+EFFECT_FLAG_DAMAGE_STEP+EFFECT_FLAG_DELAY)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET + EFFECT_FLAG_DAMAGE_STEP + EFFECT_FLAG_DELAY, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetTarget(s.eqtg)
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    // e2:SetOperation(s.eqop)
    effect::set_operation(d, e2, Some(operation(eqop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // aux.AddEREquipLimit(c,nil,aux.FilterBoolFunction(Card.IsMonster),Card.EquipByEffectAndLimitRegister,e2)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop), e2, None, None, None);
    // Destroy
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e3, aux::stringid(ID, 1));
    // e3:SetCategory(CATEGORY_DESTROY)
    effect::set_category(d, e3, CATEGORY_DESTROY);
    // e3:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e3, EFFECT_TYPE_IGNITION);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCountLimit(1)
    effect::set_count_limit(d, e3, 1, 0, 0, 0);
    // e3:SetCost(s.descost)
    effect::set_cost(d, e3, Some(cost_target(descost)));
    // e3:SetTarget(s.destg)
    effect::set_target(d, e3, Some(cost_target(destg)));
    // e3:SetOperation(s.desop)
    effect::set_operation(d, e3, Some(operation(desop)));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
/// `s.filter(c,e,tp)`
fn filter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsMonster() and c:IsPreviousLocation(LOCATION_MZONE) and c:IsPreviousControler(1-tp)
    //     and c:IsLocation(LOCATION_GRAVE) and c:IsReason(REASON_EFFECT|REASON_BATTLE) and c:IsCanBeEffectTarget(e) and not c:IsForbidden()
    card::is_monster(d, c)
        && card::is_previous_location(d, c, LOCATION_MZONE)
        && card::is_previous_controler(d, c, 1 - tp)
        && card::is_location(d, c, LOCATION_GRAVE)
        && card::is_reason(d, c, REASON_EFFECT | REASON_BATTLE)
        && card::is_can_be_effect_target(d, c, Some(e))
        && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return eg:IsContains(chkc) and s.filter(chkc,e,tp) end
    if let Some(chkc) = a.chkc() {
        let eg = a.eg.clone().nil();
        return group::is_contains(&eg, chkc) && filter(d, chkc, a.e, tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and eg:IsExists(s.filter,1,nil,e,tp) end
    if a.chk == Some(0) {
        let eg = a.eg.clone().nil();
        return duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && group::is_exists(d, &eg, &|d, c| filter(d, c, a.e, tp), 1, NO_EX);
    }
    // local g=eg:Filter(s.filter,nil,e,tp)
    let eg = a.eg.clone().nil();
    let g = group::filter(d, &eg, &|d, c| filter(d, c, a.e, tp), NO_EX);
    // local tc=nil
    let tc;
    // if #g>1 then
    if g.len() > 1 {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
        duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
        // tc=g:Select(tp,1,1,nil)
        let sel = group::select(d, &g, tp, 1, 1, false, NO_EX).nil();
        tc = group::get_first(d, &sel);
    } else {
        // tc=g:GetFirst()
        tc = group::get_first(d, &g);
    }
    // Duel.SetTargetCard(tc)
    if let Some(tc) = tc {
        duel::set_target_card(d, tc);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,tc,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, tc.map(CardSet::from_card), 1, 0, 0);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if c:IsRelateToEffect(e) and c:IsFaceup() and tc and tc:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        if let Some(tc) = tc {
            if card::is_relate_to_effect(d, tc, a.e) {
                // c:EquipByEffectAndLimitRegister(e,tp,tc)
                card::equip_by_effect_and_limit_register(d, c, a.e, tp, tc, None, None);
            }
        }
    }
}
/// `aux.FilterBoolFunction(Card.IsMonster)`
fn eqval(d: &mut Duel, a: Args) -> bool {
    card::is_monster(d, a.c(0))
}
/// `Card.EquipByEffectAndLimitRegister` as the equip operation
fn equipop(d: &mut Duel, a: Args) {
    let c = a.c(0);
    let e = a.e(1);
    let tp = a.p(2);
    let tc = a.c(3);
    card::equip_by_effect_and_limit_register(d, c, e, tp, tc, None, None);
}
/// `s.descost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn descost(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local h=e:GetHandler()
    let h = effect::get_handler(d, a.e).nil();
    // if chk==0 then return e:GetHandler():GetEquipGroup():IsExists(s.tgfilter,1,nil,tp) end
    let eq = card::get_equip_group(d, h);
    if a.chk == Some(0) {
        return group::is_exists(d, &eq, &|d, c| tgfilter(d, c, tp), 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TOGRAVE)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TOGRAVE as u64);
    // local g=e:GetHandler():GetEquipGroup():FilterSelect(tp,s.tgfilter,1,1,nil,tp)
    let g = group::filter_select(d, &eq, tp, &|d, c| tgfilter(d, c, tp), 1, 1, false, NO_EX).nil();
    // e:SetLabel(g:GetFirst():GetRace())
    let first = group::get_first(d, &g).nil();
    let race = card::get_race(d, first);
    effect::set_label(d, a.e, &[race as i64]);
    // Duel.SendtoGrave(g,REASON_COST)
    duel::send_to_grave(d, g, REASON_COST, None, None);
    true
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsAbleToGraveAsCost() and Duel.IsExistingMatchingCard(s.desfilter,tp,0,LOCATION_MZONE,1,nil,c:GetRace())
    if !card::is_able_to_grave_as_cost(d, c) {
        return false;
    }
    let rc = card::get_race(d, c);
    duel::is_existing_matching_card(d, Some(&|d, x| desfilter(d, x, rc)), tp, 0, LOCATION_MZONE, 1, NO_EX)
}
/// `s.desfilter(c,rc)`
fn desfilter(d: &mut Duel, c: CardId, rc: u64) -> bool {
    // return c:IsFaceup() and c:IsRace(rc)
    card::is_faceup(d, c) && card::is_race(d, c, rc)
}
/// `s.destg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,e:GetLabel())
    let rc = effect::get_label(d, a.e) as u64;
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, rc)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,#g,0,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), n, 0, 0);
    true
}
/// `s.desop(e,tp,eg,ep,ev,re,r,rp)`
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,0,LOCATION_MZONE,nil,e:GetLabel())
    let rc = effect::get_label(d, a.e) as u64;
    let g = duel::get_matching_group(d, Some(&|d, c| desfilter(d, c, rc)), tp, 0, LOCATION_MZONE, NO_EX);
    // Duel.Destroy(g,REASON_EFFECT)
    duel::destroy(d, g, REASON_EFFECT, None, None);
}
