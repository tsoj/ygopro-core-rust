//! Sea Lancer
use super::prelude::*;

pub const ID: u32 = 22842214;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // equip
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetProperty(EFFECT_FLAG_NO_TURN_RESET+EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_NO_TURN_RESET + EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetRange(LOCATION_MZONE)
    effect::set_range(d, e1, LOCATION_MZONE);
    // e1:SetCountLimit(1)
    effect::set_count_limit(d, e1, 1, 0, 0, 0);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // aux.AddEREquipLimit(c,nil,s.eqval,s.equipop,e1)
    aux::add_er_equip_limit(d, c, None, lua_fn(eqval), lua_fn(equipop_fn), e1, None, None, None);
    // Destroy replace
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_CONTINUOUS+EFFECT_TYPE_SINGLE)
    effect::set_type(d, e2, EFFECT_TYPE_CONTINUOUS + EFFECT_TYPE_SINGLE);
    // e2:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e2, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCode(EFFECT_DESTROY_REPLACE)
    effect::set_code(d, e2, EFFECT_DESTROY_REPLACE);
    // e2:SetTarget(s.desreptg)
    effect::set_target(d, e2, Some(cost_target(desreptg)));
    // e2:SetOperation(s.desrepop)
    effect::set_operation(d, e2, Some(operation(desrepop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // atkup
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e3, EFFECT_TYPE_SINGLE);
    // e3:SetProperty(EFFECT_FLAG_SINGLE_RANGE)
    effect::set_property(d, e3, EFFECT_FLAG_SINGLE_RANGE, 0);
    // e3:SetRange(LOCATION_MZONE)
    effect::set_range(d, e3, LOCATION_MZONE);
    // e3:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e3, EFFECT_UPDATE_ATTACK);
    // e3:SetCondition(s.atcon)
    effect::set_condition(d, e3, Some(condition(atcon)));
    // e3:SetValue(1000)
    effect::set_value(d, e3, 1000);
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
}
fn eqval(d: &mut Duel, a: Args) -> bool {
    // return ec:IsControler(tp) and ec:IsRace(RACE_FISH|RACE_AQUA|RACE_SEASERPENT)
    let ec = a.c(0);
    let tp = a.p(2);
    card::is_controler(d, ec, tp) && card::is_race(d, ec, RACE_FISH | RACE_AQUA | RACE_SEASERPENT)
}
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsRace(RACE_FISH|RACE_AQUA|RACE_SEASERPENT) and not c:IsForbidden()
    card::is_faceup(d, c) && card::is_race(d, c, RACE_FISH | RACE_AQUA | RACE_SEASERPENT) && !card::is_forbidden(d, c)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsLocation(LOCATION_REMOVED) and chkc:IsControler(tp) and s.filter(chkc) end
    if let Some(chkc) = a.chkc() {
        return card::is_location(d, chkc, LOCATION_REMOVED) && card::is_controler(d, chkc, a.tp) && filter(d, chkc);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.filter,tp,LOCATION_REMOVED,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&filter), a.tp, LOCATION_REMOVED, 0, 1, NO_EX);
    }
    // local fc=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let fc = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
    // local g=Duel.SelectTarget(tp,s.filter,tp,LOCATION_REMOVED,0,1,fc,nil)
    let g = duel::select_target(d, a.tp, Some(&filter), a.tp, LOCATION_REMOVED, 0, 1, fc as u16, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,g,#g,0,0)
    let n = g.as_ref().map_or(0, |g| g.len()) as u8;
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, g, n, 0, 0);
    true
}
/// `s.equipop(c,e,tp,tc,chk)`
fn equipop(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId, chk: bool) {
    // local eff=false or chk
    let eff = chk;
    // Duel.Equip(tp,tc,c,false,eff)
    duel::equip(d, tp, tc, c, false, eff);
    // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
    card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetProperty(EFFECT_FLAG_OWNER_RELATE)
    effect::set_property(d, e1, EFFECT_FLAG_OWNER_RELATE, 0);
    // e1:SetCode(EFFECT_EQUIP_LIMIT)
    effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
    effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
    // e1:SetValue(Card.EquipByEffectLimit)
    effect::set_value_fn(d, e1, card::equip_by_effect_limit());
    // e1:SetLabelObject(e:GetLabelObject())
    let lo = effect::get_label_object(d, e);
    effect::set_label_object(d, e1, lo);
    // tc:RegisterEffect(e1)
    card::register_effect(d, tc, e1, false);
}
/// The wrapper of `s.equipop` that `aux.AddEREquipLimit` calls: `function(c,e,tp,tc) equipop(c,e,tp,tc) end`
fn equipop_fn(d: &mut Duel, a: Args) {
    let c = a.c(0);
    let e = a.e(1);
    let tp = a.p(2);
    let tc = a.c(3);
    equipop(d, c, e, tp, tc, false);
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local g=Duel.GetTargetCards(e)
    let mut g = duel::get_target_cards(d, a.e).nil();
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    // local tg=Group.CreateGroup()
    let mut tg = CardSet::new();
    // if c:IsRelateToEffect(e) and c:IsFaceup() then
    if card::is_relate_to_effect(d, c, a.e) && card::is_faceup(d, c) {
        // if ft>=#g then
        if ft >= g.len() as i32 {
            // tg:Merge(g)
            group::add_card(&mut tg, &g);
        } else {
            // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
            duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_EQUIP as u64);
            // tg:Merge(g:Select(tp,ft,ft,nil))
            let sel = group::select(d, &g, a.tp, ft as u16, ft as u16, false, NO_EX).nil();
            group::add_card(&mut tg, &sel);
        }
    }
    // g:Sub(tg)
    group::remove_card(&mut g, &tg);
    // local tc=tg:GetFirst()
    let _ = group::get_first(d, &tg);
    // for tc in aux.Next(tg) do
    for tc in group::iter(d, &tg) {
        // s.equipop(c,e,tp,tc,true)
        equipop(d, c, a.e, a.tp, tc, true);
    }
    // Duel.EquipComplete()
    duel::equip_complete(d);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.SendtoGrave(g,REASON_RULE)
        duel::send_to_grave(d, g, REASON_RULE, None, None);
    }
}
fn eqfilter(d: &mut Duel, c: CardId, ec: CardId) -> bool {
    // return c:GetFlagEffect(id)~=0 and c:IsHasCardTarget(ec)
    card::get_flag_effect(d, c, ID) != 0 && card::is_has_card_target(d, c, ec)
}
fn repfilter(d: &mut Duel, c: CardId, e: EffectId, ec: CardId) -> bool {
    // return s.eqfilter(c,ec) and c:IsDestructable(e) and not c:IsStatus(STATUS_DESTROY_CONFIRMED)
    eqfilter(d, c, ec) && card::is_destructable(d, c, Some(e)) && !card::is_status(d, c, STATUS_DESTROY_CONFIRMED)
}
fn desreptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return not c:IsReason(REASON_REPLACE) and Duel.IsExistingMatchingCard(s.repfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,nil,e,c) end
    if a.chk == Some(0) {
        return !card::is_reason(d, c, REASON_REPLACE) && duel::is_existing_matching_card(d, Some(&|d, x| repfilter(d, x, a.e, c)), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX);
    }
    // if Duel.SelectEffectYesNo(tp,c,96) then
    if duel::select_effect_yes_no(d, a.tp, c, Some(96)) {
        // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESREPLACE)
        duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_DESREPLACE as u64);
        // local tc=Duel.SelectMatchingCard(tp,s.repfilter,tp,LOCATION_SZONE,LOCATION_SZONE,1,1,nil,e,c):GetFirst()
        let sel = duel::select_matching_card(d, a.tp, Some(&|d, x| repfilter(d, x, a.e, c)), a.tp, LOCATION_SZONE, LOCATION_SZONE, 1, 1, false, NO_EX).nil();
        let tc = group::get_first(d, &sel);
        // e:SetLabelObject(tc)
        effect::set_label_object(d, a.e, match tc {
            Some(x) => Value::Card(x),
            None => Value::Nil,
        });
        // tc:SetStatus(STATUS_DESTROY_CONFIRMED,true)
        let tc = tc.nil();
        card::set_status(d, tc, STATUS_DESTROY_CONFIRMED, true);
        // return true
        true
    } else {
        // return false
        false
    }
}
fn desrepop(d: &mut Duel, a: &EffArgs) {
    // local g=e:GetLabelObject()
    let g = effect::get_label_object(d, a.e).as_card().nil();
    // g:SetStatus(STATUS_DESTROY_CONFIRMED,false)
    card::set_status(d, g, STATUS_DESTROY_CONFIRMED, false);
    // Duel.Destroy(g,REASON_EFFECT|REASON_REPLACE)
    duel::destroy(d, g, REASON_EFFECT | REASON_REPLACE, None, None);
}
fn atcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return Duel.IsExistingMatchingCard(s.eqfilter,e:GetHandlerPlayer(),LOCATION_SZONE,LOCATION_SZONE,1,nil,e:GetHandler())
    let hp = effect::get_handler_player(d, a.e);
    let h = effect::get_handler(d, a.e).nil();
    duel::is_existing_matching_card(d, Some(&|d, x| eqfilter(d, x, h)), hp, LOCATION_SZONE, LOCATION_SZONE, 1, NO_EX)
}
