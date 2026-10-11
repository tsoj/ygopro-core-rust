//! The League of Uniform Nomenclature Strikes
//! 同姓同名同盟罷業
use super::prelude::*;

pub const ID: u32 = 29265962;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Target 1 face-up monster you control; equip 2 monsters with the same original name as that target from your hand, Deck, and/or GY to that target as Equip Spells, and if you do, it cannot attack or be destroyed by battle while equipped with those 2 cards. You cannot Special Summon for the rest of this turn after this card resolves, except monsters with the same original Type as the targeted monster
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_EQUIP)
    effect::set_category(d, e1, CATEGORY_EQUIP);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e1, EFFECT_FLAG_CARD_TARGET, 0);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetTarget(s.eqtg)
    effect::set_target(d, e1, Some(cost_target(eqtg)));
    // e1:SetOperation(s.eqop)
    effect::set_operation(d, e1, Some(operation(eqop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
/// `s.tgfilter(c,tp)`
fn tgfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsFaceup() and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,2,nil,tp,c:GetOriginalCodeRule())
    let code = card::get_original_code_rule(d, c)[0];
    card::is_faceup(d, c)
        && duel::is_existing_matching_card(d, Some(&|d, cc| eqfilter(d, cc, tp, code)), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 2, NO_EX)
}
/// `s.eqfilter(c,tp,code)`
fn eqfilter(d: &mut Duel, c: CardId, tp: u8, code: u32) -> bool {
    // return c:IsOriginalCodeRule(code) and c:CheckUniqueOnField(tp) and not c:IsForbidden()
    card::is_original_code_rule(d, c, &[code]) && card::check_unique_on_field(d, c, tp, None, None) && !card::is_forbidden(d, c)
}
/// `s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)`
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // if chkc then return chkc:IsControler(tp) and chkc:IsLocation(LOCATION_MZONE) and s.tgfilter(chkc,tp) end
    if let Some(chkc) = a.chkc() {
        return card::is_controler(d, chkc, tp) && card::is_location(d, chkc, LOCATION_MZONE) && tgfilter(d, chkc, tp);
    }
    // if chk==0 then
    if a.chk == Some(0) {
        // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
        let mut ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
        // if e:GetHandler():IsLocation(LOCATION_HAND) then ft=ft-1 end
        if card::is_location(d, effect::get_handler(d, a.e).nil(), LOCATION_HAND) {
            ft -= 1;
        }
        // return ft>=2 and Duel.IsExistingTarget(s.tgfilter,tp,LOCATION_MZONE,0,1,nil,tp)
        return ft >= 2 && duel::is_existing_target(d, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, NO_EX);
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TARGET)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TARGET as u64);
    // Duel.SelectTarget(tp,s.tgfilter,tp,LOCATION_MZONE,0,1,1,nil,tp)
    duel::select_target(d, tp, Some(&|d, c| tgfilter(d, c, tp)), tp, LOCATION_MZONE, 0, 1, 1, false, NO_EX);
    // Duel.SetOperationInfo(0,CATEGORY_EQUIP,nil,2,tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE)
    duel::set_operation_info(d, 0, CATEGORY_EQUIP, None, 2, tp, (LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE) as i32);
    true
}
/// `s.eqop(e,tp,eg,ep,ev,re,r,rp)`
fn eqop(d: &mut Duel, a: &EffArgs) {
    let e = a.e;
    let tp = a.tp;
    // local tc=Duel.GetFirstTarget()
    let tc = duel::first_target(d);
    // if tc:IsFacedown() or not tc:IsRelateToEffect(e) then return end
    if card::is_facedown(d, tc.nil()) || !card::is_relate_to_effect(d, tc.nil(), e) {
        return;
    }
    // local c=e:GetHandler()
    let c = effect::get_handler(d, e).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_EQUIP)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_EQUIP as u64);
    // local eqg=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.eqfilter),tp,LOCATION_HAND|LOCATION_DECK|LOCATION_GRAVE,0,2,2,nil,tp,tc:GetOriginalCodeRule())
    let code = card::get_original_code_rule(d, tc.nil())[0];
    let nv = aux::necro_valley_filter(move |d, cc| eqfilter(d, cc, tp, code));
    let eqg = duel::select_matching_card(d, tp, Some(&nv), tp, LOCATION_HAND | LOCATION_DECK | LOCATION_GRAVE, 0, 2, 2, false, NO_EX).nil();
    // if #eqg==2 then
    if eqg.len() == 2 {
        // local fid=e:GetFieldID()
        let fid = effect::get_field_id(d, e);
        // local equip_success=false
        let mut equip_success = false;
        // for ec in eqg:Iter() do
        for ec in group::iter(d, &eqg) {
            // if Duel.Equip(tp,ec,tc,true,true) then
            if duel::equip(d, tp, ec, tc.nil(), true, true) {
                // equip_success=true
                equip_success = true;
                // ec:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,1,fid)
                card::register_flag_effect(d, ec, ID, RESET_EVENT | RESETS_STANDARD, 0, 1, fid as i64, 0);
                // --Equip limit
                // local e1=Effect.CreateEffect(ec)
                let e1 = effect::create_effect(d, ec);
                // e1:SetType(EFFECT_TYPE_SINGLE)
                effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
                // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE)
                effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE, 0);
                // e1:SetCode(EFFECT_EQUIP_LIMIT)
                effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
                // e1:SetValue(function(e,c) return c==tc end)
                let tcc = tc.nil();
                effect::set_value_fn(d, e1, lua_fn(move |_d, a| a.c(1) == tcc));
                // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
                effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
                // ec:RegisterEffect(e1)
                card::register_effect(d, ec, e1, false);
            }
        }
        // Duel.EquipComplete()
        duel::equip_complete(d);
        // if equip_success then
        if equip_success {
            // --It cannot attack or be destroyed by battle while equipped with those 2 cards
            // local e2=Effect.CreateEffect(c)
            let e2 = effect::create_effect(d, c);
            // e2:SetDescription(aux.Stringid(id,1))
            effect::set_description(d, e2, aux::stringid(ID, 1));
            // e2:SetType(EFFECT_TYPE_SINGLE)
            effect::set_type(d, e2, EFFECT_TYPE_SINGLE);
            // e2:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_CLIENT_HINT)
            effect::set_property(d, e2, EFFECT_FLAG_CANNOT_DISABLE | EFFECT_FLAG_CLIENT_HINT, 0);
            // e2:SetCode(EFFECT_CANNOT_ATTACK)
            effect::set_code(d, e2, EFFECT_CANNOT_ATTACK);
            // e2:SetCondition(function(e) return e:GetHandler():GetEquipGroup():IsExists(function(ec) return ec:GetFlagEffectLabel(id)==fid end,2,nil) end)
            effect::set_condition(
                d,
                e2,
                Some(condition(move |d, a| {
                    let h = effect::get_handler(d, a.e).nil();
                    let eqg2 = card::get_equip_group(d, h);
                    group::is_exists(d, &eqg2, &|d, ec| card::get_flag_effect_label(d, ec, ID).first() == Some(&(fid as i64)), 2, NO_EX)
                })),
            );
            // e2:SetReset(RESET_EVENT|RESETS_STANDARD)
            effect::set_reset(d, e2, RESET_EVENT | RESETS_STANDARD, 1);
            // tc:RegisterEffect(e2)
            card::register_effect(d, tc.nil(), e2, false);
            // local e3=e2:Clone()
            let e3 = effect::clone(d, e2);
            // e3:SetCode(EFFECT_INDESTRUCTABLE_BATTLE)
            effect::set_code(d, e3, EFFECT_INDESTRUCTABLE_BATTLE);
            // e3:SetValue(1)
            effect::set_value(d, e3, 1);
            // tc:RegisterEffect(e3)
            card::register_effect(d, tc.nil(), e3, false);
        }
    }
    // if not e:IsHasType(EFFECT_TYPE_ACTIVATE) then return end
    if !effect::is_has_type(d, e, EFFECT_TYPE_ACTIVATE) {
        return;
    }
    // local orig_race=tc:GetOriginalRace()
    let orig_race = card::get_original_race(d, tc.nil());
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,2))
    effect::set_description(d, e4, aux::stringid(ID, 2));
    // e4:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD);
    // e4:SetProperty(EFFECT_FLAG_PLAYER_TARGET+EFFECT_FLAG_CLIENT_HINT)
    effect::set_property(d, e4, EFFECT_FLAG_PLAYER_TARGET | EFFECT_FLAG_CLIENT_HINT, 0);
    // e4:SetCode(EFFECT_CANNOT_SPECIAL_SUMMON)
    effect::set_code(d, e4, EFFECT_CANNOT_SPECIAL_SUMMON);
    // e4:SetTargetRange(1,0)
    effect::set_target_range(d, e4, 1, 0);
    // e4:SetTarget(function(e,c) return not c:IsOriginalRace(orig_race) end)
    effect::set_target(d, e4, Some(effect_card_fn(move |d, _e, c| !card::is_original_race(d, c, orig_race))));
    // e4:SetReset(RESET_PHASE|PHASE_END)
    effect::set_reset(d, e4, RESET_PHASE | PHASE_END, 1);
    // Duel.RegisterEffect(e4,tp)
    duel::register_effect(d, e4, tp);
}
