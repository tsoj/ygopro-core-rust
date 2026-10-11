//! Renewal of the World
use super::prelude::*;

pub const ID: u32 = 81306186;

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e1, CATEGORY_REMOVE);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.rmtg)
    effect::set_target(d, e1, Some(cost_target(rmtg)));
    // e1:SetOperation(s.rmop)
    effect::set_operation(d, e1, Some(operation(rmop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Ritual Summon or add to the hand
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_QUICK_O)
    effect::set_type(d, e2, EFFECT_TYPE_QUICK_O);
    // e2:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e2, EVENT_FREE_CHAIN);
    // e2:SetRange(LOCATION_SZONE)
    effect::set_range(d, e2, LOCATION_SZONE);
    // e2:SetCountLimit(1,id)
    effect::set_count_limit(d, e2, 1, ID, 0, 0);
    // e2:SetLabelObject(e1)
    effect::set_label_object(d, e2, Value::Effect(e1));
    // e2:SetCost(s.cost)
    effect::set_cost(d, e2, Some(cost_target(cost_fn)));
    // e2:SetTarget(s.target)
    effect::set_target(d, e2, Some(cost_target(target)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
/// `s.filter(c)`
fn filter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsRitualMonster() and c:IsAbleToRemove()
    card::is_ritual_monster(d, c) && card::is_able_to_remove(d, c, None, None, None)
}
/// `s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.IsExistingMatchingCard(s.filter,tp,LOCATION_DECK,0,1,nil) end
    if a.chk == Some(0) {
        return duel::is_existing_matching_card(d, Some(&filter), a.tp, LOCATION_DECK, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,nil,1,tp,LOCATION_DECK)
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, None, 1, a.tp, LOCATION_DECK as i32);
    true
}
/// `s.rmop(e,tp,eg,ep,ev,re,r,rp)`
fn rmop(d: &mut Duel, a: &EffArgs) {
    // if not e:GetHandler():IsRelateToEffect(e) then return end
    let h = effect::get_handler(d, a.e).nil();
    if !card::is_relate_to_effect(d, h, a.e) {
        return;
    }
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_REMOVE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_REMOVE as u64);
    // local tc=Duel.SelectMatchingCard(tp,s.filter,tp,LOCATION_DECK,0,1,1,nil):GetFirst()
    let sel = duel::select_matching_card(d, a.tp, Some(&filter), a.tp, LOCATION_DECK, 0, 1, 1, false, NO_EX).nil();
    let tc = group::get_first(d, &sel);
    // if tc and Duel.Remove(tc,POS_FACEUP,REASON_EFFECT)~=0 then
    if let Some(tc) = tc {
        if duel::remove(d, tc, POS_FACEUP as u8, REASON_EFFECT, None, None) != 0 {
            // tc:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD,0,0)
            card::register_flag_effect(d, tc, ID, RESET_EVENT | RESETS_STANDARD, 0, 0, 0, 0);
            // e:SetLabelObject(tc)
            effect::set_label_object(d, a.e, Value::Card(tc));
        }
    }
}
/// `s.cost(e,tp,eg,ep,ev,re,r,rp,chk)`
fn cost_fn(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return c:IsAbleToGraveAsCost() and c:IsStatus(STATUS_EFFECT_ENABLED) end
    if a.chk == Some(0) {
        return card::is_able_to_grave_as_cost(d, c) && card::is_status(d, c, STATUS_EFFECT_ENABLED);
    }
    // Duel.SendtoGrave(c,REASON_COST)
    duel::send_to_grave(d, c, REASON_COST, None, None);
    true
}
/// `s.relfilter(c,e,tp,tc,ft)`
fn relfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, tc: CardId, ft: i32) -> bool {
    // local lv=tc:GetLevel()
    let lv = card::get_level(d, tc);
    // aux.RitualSummoningLevel=lv
    set_lua_global(d, "RitualSummoningLevel", Value::Int(lv as i64));
    // local mlv=c:GetRitualLevel(tc)
    let mlv = card::get_ritual_level(d, c, tc);
    // aux.RitualSummoningLevel=nil
    set_lua_global(d, "RitualSummoningLevel", Value::Nil);
    // if not ((mlv&0xffff)>=lv or (mlv>>16)>=lv) then return false end
    if !((mlv & 0xffff) >= lv || (mlv >> 16) >= lv) {
        return false;
    }
    // if tc.mat_filter and not tc.mat_filter(c) or (tc.ritual_custom_check and not tc.ritual_custom_check(e,tp,Group.FromCards(c),tc)) then return false end
    let info = crate::card_info(d, tc);
    if let Some(mf) = info.mat_filter {
        if !mf(d, c, tp) {
            return false;
        }
    }
    if let Some(rcc) = info.ritual_custom_check {
        if !rcc(d, e, tp, &CardSet::from_card(c), tc).0 {
            return false;
        }
    }
    // if c:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, c, LOCATION_GRAVE) {
        // return c:IsType(TYPE_RITUAL) and ft>0 and c:IsAbleToDeck()
        card::is_type(d, c, TYPE_RITUAL) && ft > 0 && card::is_able_to_deck(d, c)
    } else {
        // return (ft>0 or c:IsControler(tp)) and c:IsReleasableByEffect(e)
        (ft > 0 || card::is_controler(d, c, tp)) && card::is_releasable_by_effect(d, c, Some(e), None)
    }
}
/// `s.target(e,tp,eg,ep,ev,re,r,rp,chk)`
fn target(d: &mut Duel, a: &EffArgs) -> bool {
    // local tc=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let tc = effect::get_label_object(d, e1).as_card();
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // if chk==0 then
    if a.chk == Some(0) {
        // if not tc or tc:IsFacedown() or tc:GetFlagEffect(id)==0 then return false end
        let Some(tc) = tc else {
            return false;
        };
        if card::is_facedown(d, tc) || card::get_flag_effect(d, tc, ID) == 0 {
            return false;
        }
        // local b1=tc:IsCanBeSpecialSummoned(e,SUMMON_TYPE_RITUAL,tp,false,true)
        //     and Duel.IsExistingMatchingCard(s.relfilter,tp,LOCATION_MZONE|LOCATION_HAND|LOCATION_GRAVE,0,1,tc,e,tp,tc,ft)
        let b1 = card::is_can_be_special_summoned(d, tc, a.e, SUMMON_TYPE_RITUAL, a.tp, false, true, None, None, None)
            && duel::is_existing_matching_card(
                d,
                Some(&|d, x| relfilter(d, x, a.e, a.tp, tc, ft)),
                a.tp,
                LOCATION_MZONE | LOCATION_HAND | LOCATION_GRAVE,
                0,
                1,
                tc.into(),
            );
        // local b2=tc:IsAbleToHand()
        let b2 = card::is_able_to_hand(d, tc, None);
        // return b1 or b2
        return b1 || b2;
    }
    // local b1=tc:IsCanBeSpecialSummoned(e,SUMMON_TYPE_RITUAL,tp,false,true)
    //     and Duel.IsExistingMatchingCard(s.relfilter,tp,LOCATION_MZONE|LOCATION_HAND|LOCATION_GRAVE,0,1,tc,e,tp,tc,ft)
    let tc = tc.nil();
    let b1 = card::is_can_be_special_summoned(d, tc, a.e, SUMMON_TYPE_RITUAL, a.tp, false, true, None, None, None)
        && duel::is_existing_matching_card(
            d,
            Some(&|d, x| relfilter(d, x, a.e, a.tp, tc, ft)),
            a.tp,
            LOCATION_MZONE | LOCATION_HAND | LOCATION_GRAVE,
            0,
            1,
            tc.into(),
        );
    // local b2=tc:IsAbleToHand()
    let b2 = card::is_able_to_hand(d, tc, None);
    // local sel=0
    // if b1 and b2 then
    //     sel=Duel.SelectOption(tp,aux.Stringid(id,0),aux.Stringid(id,1))
    // elseif b1 then
    //     sel=Duel.SelectOption(tp,aux.Stringid(id,0))
    // else
    //     sel=Duel.SelectOption(tp,aux.Stringid(id,1))+1
    // end
    let sel = if b1 && b2 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0), aux::stringid(ID, 1)])
    } else if b1 {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 0)])
    } else {
        duel::select_option(d, a.tp, true, &[aux::stringid(ID, 1)]) + 1
    };
    // if sel==0 then
    if sel == 0 {
        // e:SetCategory(CATEGORY_TODECK+CATEGORY_SPECIAL_SUMMON)
        effect::set_category(d, a.e, CATEGORY_TODECK + CATEGORY_SPECIAL_SUMMON);
        // e:SetOperation(s.spop)
        effect::set_operation(d, a.e, Some(operation(spop)));
        // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,tc,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(tc)), 1, a.tp, 0);
        // Duel.SetOperationInfo(0,CATEGORY_TODECK,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TODECK, None, 1, a.tp, LOCATION_GRAVE as i32);
    } else {
        // e:SetCategory(CATEGORY_TOHAND)
        effect::set_category(d, a.e, CATEGORY_TOHAND);
        // e:SetOperation(s.thop)
        effect::set_operation(d, a.e, Some(operation(thop)));
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,tc,1,0,0)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, Some(CardSet::from_card(tc)), 1, 0, 0);
    }
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local rc=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let rc = effect::get_label_object(d, e1).as_card();
    // if not rc or rc:IsFacedown() or rc:GetFlagEffect(id)==0 then return end
    let Some(rc) = rc else {
        return;
    };
    if card::is_facedown(d, rc) || card::get_flag_effect(d, rc, ID) == 0 {
        return;
    }
    // if not rc:IsCanBeSpecialSummoned(e,SUMMON_TYPE_RITUAL,tp,false,true) then return end
    if !card::is_can_be_special_summoned(d, rc, a.e, SUMMON_TYPE_RITUAL, a.tp, false, true, None, None, None) {
        return;
    }
    // local ft=Duel.GetLocationCount(tp,LOCATION_MZONE)
    let ft = duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_RELEASE)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, HINTMSG_RELEASE as u64);
    // local tc=Duel.SelectMatchingCard(tp,aux.NecroValleyFilter(s.relfilter),tp,LOCATION_MZONE|LOCATION_HAND|LOCATION_GRAVE,0,1,1,rc,e,tp,rc,ft):GetFirst()
    let nv = aux::necro_valley_filter(|d, x| relfilter(d, x, a.e, a.tp, rc, ft));
    let sel = duel::select_matching_card(d, a.tp, Some(&nv), a.tp, LOCATION_MZONE | LOCATION_HAND | LOCATION_GRAVE, 0, 1, 1, false, rc.into()).nil();
    let tc = group::get_first(d, &sel);
    // if tc then
    if let Some(tc) = tc {
        // rc:SetMaterial(Group.FromCards(tc))
        card::set_material(d, rc, CardSet::from_card(tc));
        // if tc:IsLocation(LOCATION_GRAVE) then
        if card::is_location(d, tc, LOCATION_GRAVE) {
            // if Duel.SendtoDeck(tc,nil,SEQ_DECKSHUFFLE,REASON_EFFECT+REASON_MATERIAL+REASON_RITUAL)==0 then return end
            if duel::send_to_deck(d, tc, None, SEQ_DECKSHUFFLE as i32, REASON_EFFECT | REASON_MATERIAL | REASON_RITUAL, None) == 0 {
                return;
            }
        } else {
            // if Duel.Release(tc,REASON_EFFECT+REASON_MATERIAL+REASON_RITUAL)==0 then return end
            if duel::release(d, tc, REASON_EFFECT | REASON_MATERIAL | REASON_RITUAL, None) == 0 {
                return;
            }
        }
        // Duel.SpecialSummon(rc,SUMMON_TYPE_RITUAL,tp,tp,false,true,POS_FACEUP)
        duel::special_summon(d, rc, SUMMON_TYPE_RITUAL, a.tp, a.tp, false, true, POS_FACEUP as u8, None);
        // rc:CompleteProcedure()
        card::complete_procedure(d, rc);
    }
}
/// `s.thop(e,tp,eg,ep,ev,re,r,rp)`
fn thop(d: &mut Duel, a: &EffArgs) {
    // local rc=e:GetLabelObject():GetLabelObject()
    let e1 = effect::get_label_object(d, a.e).as_effect().nil();
    let rc = effect::get_label_object(d, e1).as_card();
    // if not rc or rc:IsFacedown() or rc:GetFlagEffect(id)==0 then return end
    let Some(rc) = rc else {
        return;
    };
    if card::is_facedown(d, rc) || card::get_flag_effect(d, rc, ID) == 0 {
        return;
    }
    // Duel.SendtoHand(rc,nil,REASON_EFFECT)
    duel::send_to_hand(d, rc, None, REASON_EFFECT, None);
}
