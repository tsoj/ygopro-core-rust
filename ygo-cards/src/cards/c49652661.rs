//! 武器庫整理
//! Arsenal Audit
use super::prelude::*;

pub const ID: u32 = 49652661;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Send up to 2 Equip Spells with different names from your Deck to the GY
    let e1 = effect::create_effect(d, c);
    effect::set_description(d, e1, aux::stringid(ID, 0));
    effect::set_category(d, e1, CATEGORY_TOGRAVE);
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    effect::set_target(d, e1, Some(cost_target(tgtg)));
    effect::set_operation(d, e1, Some(operation(tgop)));
    effect::set_hint_timing(d, e1, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e1, false);

    // Equip 1 face-up monster you control with up to 2 Equip Spells with different names from your hand and/or GY that can equip to it, but any battle damage it inflicts to your opponent is halved while equipped with any of those Equip Spells
    let e2 = effect::create_effect(d, c);
    effect::set_description(d, e2, aux::stringid(ID, 1));
    effect::set_category(d, e2, CATEGORY_EQUIP);
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    effect::set_range(d, e2, LOCATION_GRAVE);
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    effect::set_cost(d, e2, Some(cost::self_banish()));
    effect::set_target(d, e2, Some(cost_target(eqtg)));
    effect::set_operation(d, e2, Some(operation(eqop)));
    effect::set_hint_timing(d, e2, 0, Some(TIMING_STANDBY_PHASE | TIMING_MAIN_END | TIMINGS_CHECK_MONSTER_E));
    card::register_effect(d, c, e2, false);
}
fn tgfilter(d: &mut Duel, c: CardId) -> bool {
    // aux.AND(Card.IsEquipSpell,Card.IsAbleToGrave)
    card::is_equip_spell(d, c) && card::is_able_to_grave(d, c, None)
}
fn tgtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(aux.AND(Card.IsEquipSpell,Card.IsAbleToGrave),tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_TOGRAVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_TOGRAVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
fn tgop(d: &mut Duel, a: &EffArgs) {
    // local g=Duel.GetMatchingGroup(aux.AND(Card.IsEquipSpell,Card.IsAbleToGrave),tp,LOCATION_DECK,0,nil)
    let g = duel::get_matching_group(d, Some(&tgfilter), a.tp, LOCATION_DECK, 0, NO_EX);
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local sg=aux.SelectUnselectGroup(g,e,tp,1,2,aux.dncheck,1,tp,HINTMSG_TOGRAVE)
    let dn = aux::dncheck();
    let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(2), Some(&dn), 1, a.tp, Some(HINTMSG_TOGRAVE as u64), None, None, false).unwrap_or_default();
    // if #sg>0 then
    if !sg.is_empty() {
        // Duel.SendtoGrave(sg,REASON_EFFECT)
        duel::send_to_grave(d, sg, REASON_EFFECT, None, None);
    }
}
fn eqtargetfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // c:IsFaceup() and Duel.IsExistingMatchingCard(s.eqspellfilter,tp,LOCATION_HAND|LOCATION_GRAVE,0,1,nil,c)
    card::is_faceup(d, c) && duel::is_existing_matching_card(d, Some(&|d, x| eqspellfilter(d, x, c)), tp, LOCATION_HAND | LOCATION_GRAVE, 0, 1, NO_EX)
}
fn eqspellfilter(d: &mut Duel, c: CardId, tc: CardId) -> bool {
    // c:IsEquipSpell() and c:CheckEquipTarget(tc)
    card::is_equip_spell(d, c) && card::check_equip_target(d, c, tc)
}
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.eqtargetfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, a.tp) && card::is_location(d, chkc, LOCATION_MZONE) && eqtargetfilter(d, chkc, a.tp);
    }
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingTarget(s.eqtargetfilter,tp,LOCATION_MZONE,0,1,nil,tp) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_target(d, Some(&|d, c| eqtargetfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.eqtargetfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, a.tp, Some(&|d, c| eqtargetfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    true
}
fn eqop(d: &mut Duel, a: &EffArgs) {
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    // if ft<=0 then return end
    let mut ft = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0;
    if ft <= 0 {
        return;
    }
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d).nil();
    // if tc:IsRelateToEffect(e) and tc:IsFaceup() then
    if card::is_relate_to_effect(d, tc, a.e) && card::is_faceup(d, tc) {
        // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.eqspellfilter),tp,LOCATION_HAND|LOCATION_GRAVE,0,nil,tc)
        let f = aux::necro_valley_filter(move |d, c| eqspellfilter(d, c, tc));
        let g = duel::get_matching_group(d, Some(&f), a.tp, LOCATION_HAND | LOCATION_GRAVE, 0, NO_EX);
        // if #g==0 then return end
        if g.is_empty() {
            return;
        }
        // ft=math.min(2,ft)
        ft = ft.min(2);
        // local sg=aux.SelectUnselectGroup(g,e,tp,1,ft,aux.dncheck,1,tp,HINTMSG_EQUIP)
        let dn = aux::dncheck();
        let sg = aux::select_unselect_group(d, &g, Some(a.e), a.tp, Some(1), Some(ft as usize), Some(&dn), 1, a.tp, Some(HINTMSG_EQUIP as u64), None, None, false).unwrap_or_default();
        // if #sg==0 then return end
        if sg.is_empty() {
            return;
        }
        // local fid=e:GetFieldID()
        let fid = effect::get_field_id(d, a.e);
        // local equip_success=false
        let mut equip_success = false;
        // for ec in sg:Iter() do
        for ec in group::iter(d, &sg) {
            // if Duel.Equip(tp,ec,tc,true,true) then
            if duel::equip(d, a.tp, ec, tc, true, true) {
                // equip_success=true
                equip_success = true;
                // ec:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,EFFECT_FLAG_CLIENT_HINT,1,fid,aux.Stringid(id,2))
                card::register_flag_effect(d, ec, ID, RESET_EVENT | RESETS_STANDARD, EFFECT_FLAG_CLIENT_HINT, 1, fid as i64, aux::stringid(ID, 2));
            }
        }
        // Duel.EquipComplete()
        duel::equip_complete(d);
        // if equip_success then
        if equip_success {
            // local e1=Effect.CreateEffect(e:GetHandler())
            let h = effect::get_handler(d, a.e).nil();
            let e1 = effect::create_effect(d, h);
            // e1:SetDescription(aux.Stringid(id,3))
            effect::set_description(d, e1, aux::stringid(ID, 3));
            // e1:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
            // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_CLIENT_HINT, 0);
            // e1:SetCode(EFFECT_CHANGE_BATTLE_DAMAGE)
            effect::set_code(d, e1, EFFECT_CHANGE_BATTLE_DAMAGE);
            // e1:SetCondition(function(e) return e:GetHandler():GetEquipGroup():IsExists(function(ec) return ec:GetFlagEffectLabel(id)==fid end,1,nil) end)
            effect::set_condition(d, e1, Some(condition(move |d, a| {
                let h = effect::get_handler(d, a.e).nil();
                let eqg = card::get_equip_group(d, h);
                group::is_exists(d, &eqg, &|d, ec| card::get_flag_effect_label(d, ec, ID).first().copied() == Some(fid as i64), 1, NO_EX)
            })));
            // e1:SetValue(aux.ChangeBattleDamage(1,HALF_DAMAGE))
            effect::set_value_fn(d, e1, aux::change_battle_damage(1, HALF_DAMAGE as i64));
            // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e1)
            card::register_effect(d, tc, e1, false);
        }
    }
}
