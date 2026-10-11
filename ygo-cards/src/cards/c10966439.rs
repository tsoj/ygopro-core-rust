//! Marshmao☆Yummy
//! マシュマオ☆ヤミー
use super::prelude::*;

pub const ID: u32 = 10966439;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_YUMMY], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Special Summon this card from your hand
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(condition(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Add 1 "Yummy" Spell/Trap from your GY to your hand, or, if this card was Special Summoned by the effect of a Synchro Monster, ... instead
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_TOHAND)
    effect::set_category(d, e2a, CATEGORY_TOHAND);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_O);
    // e2a:SetProperty(EFFECT_FLAG_DELAY)
    effect::set_property(d, e2a, EFFECT_FLAG_DELAY, 0);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetTarget(s.thpltg)
    effect::set_target(d, e2a, Some(cost_target(thpltg)));
    // e2a:SetOperation(s.thplop)
    effect::set_operation(d, e2a, Some(operation(thplop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
/// `s.spconfilter(c)`
fn spconfilter(d: &mut Duel, c: CardId) -> bool {
    // return not (c:IsAttribute(ATTRIBUTE_LIGHT) and c:IsRace(RACE_BEAST) and c:IsFaceup())
    !(card::is_attribute(d, c, ATTRIBUTE_LIGHT) && card::is_race(d, c, RACE_BEAST) && card::is_faceup(d, c))
}
/// `s.spcon(e,tp,eg,ep,ev,re,r,rp)`
fn spcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return not Duel.IsExistingMatchingCard(s.spconfilter,tp,LOCATION_MZONE,0,1,nil)
    !duel::is_existing_matching_card(d, Some(&spconfilter), a.tp, LOCATION_MZONE, 0, 1, NO_EX)
}
/// `s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, a.tp, false, false, None, None, None);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, a.tp, 0);
    true
}
/// `s.spop(e,tp,eg,ep,ev,re,r,rp)`
fn spop(d: &mut Duel, a: &EffArgs) {
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // if c:IsRelateToEffect(e) then
    if card::is_relate_to_effect(d, c, a.e) {
        // Duel.SpecialSummon(c,0,tp,tp,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, a.tp, a.tp, false, false, POS_FACEUP as u8, None);
    }
}
/// `s.thfilter(c)`
fn thfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_YUMMY) and c:IsSpellTrap() and c:IsAbleToHand()
    card::is_set_card(d, c, &[SET_YUMMY as u16]) && card::is_spell_trap(d, c) && card::is_able_to_hand(d, c, None)
}
/// `s.plfilter(c,tp,szone_chk)`
fn plfilter(d: &mut Duel, c: CardId, tp: u8, szone_chk: bool) -> bool {
    // return c:IsSetCard(SET_YUMMY) and (c:IsFieldSpell() or (szone_chk and c:IsContinuousSpellTrap()))
    //   and (c:IsFaceup() or c:IsLocation(LOCATION_DECK)) and not c:IsForbidden() and c:CheckUniqueOnField(tp)
    card::is_set_card(d, c, &[SET_YUMMY as u16])
        && (card::is_field_spell(d, c) || (szone_chk && card::is_continuous_spell_trap(d, c)))
        && (card::is_faceup(d, c) || card::is_location(d, c, LOCATION_DECK))
        && !card::is_forbidden(d, c)
        && card::check_unique_on_field(d, c, tp, None, None)
}
/// `s.thpltg(e,tp,eg,ep,ev,re,r,rp,chk)`
fn thpltg(d: &mut Duel, a: &EffArgs) -> bool {
    // local sp_chk=re and e:GetHandler():IsSpecialSummoned() and re:IsMonsterEffect() and re:GetHandler():IsOriginalType(TYPE_SYNCHRO)
    let sp_chk = match a.re {
        None => false,
        Some(re) => {
            let h = effect::get_handler(d, a.e).nil();
            card::is_special_summoned(d, h)
                && effect::is_monster_effect(d, re)
                && {
                    let rh = effect::get_handler(d, re).nil();
                    card::is_original_type(d, rh, TYPE_SYNCHRO)
                }
        }
    };
    // if chk==0 then return Duel.IsExistingMatchingCard(s.thfilter,tp,LOCATION_GRAVE,0,1,nil)
    //   or (sp_chk and Duel.IsExistingMatchingCard(s.plfilter,tp,LOCATION_DECK|LOCATION_REMOVED,0,1,nil,tp,Duel.GetLocationCount(tp,LOCATION_SZONE)>0)) end
    if a.chk == Some(0) {
        if duel::is_existing_matching_card(d, Some(&thfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX) {
            return true;
        }
        return sp_chk && {
            let szone = duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
            duel::is_existing_matching_card(d, Some(&|d, c| plfilter(d, c, a.tp, szone)), a.tp, LOCATION_DECK | LOCATION_REMOVED, 0, 1, NO_EX)
        };
    }
    // e:SetLabel(sp_chk and 1 or 0)
    effect::set_label(d, a.e, &[if sp_chk { 1 } else { 0 }]);
    // if sp_chk then
    if sp_chk {
        // Duel.SetPossibleOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_possible_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_TOHAND,nil,1,tp,LOCATION_GRAVE)
        duel::set_operation_info(d, 0, CATEGORY_TOHAND, None, 1, a.tp, LOCATION_GRAVE as i32);
    }
    true
}
/// `s.thplop(e,tp,eg,ep,ev,re,r,rp)`
fn thplop(d: &mut Duel, a: &EffArgs) {
    // local sp_chk=e:GetLabel()==1
    let sp_chk = effect::get_label(d, a.e) == 1;
    // local szone_chk=sp_chk and Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    let szone_chk = sp_chk && duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0;
    // local g=Duel.GetMatchingGroup(aux.NecroValleyFilter(s.thfilter),tp,LOCATION_GRAVE,0,nil)
    let nvf = aux::necro_valley_filter(|d, c| thfilter(d, c));
    let mut g = duel::get_matching_group(d, Some(&nvf), a.tp, LOCATION_GRAVE, 0, NO_EX);
    // if sp_chk then g:Merge(Duel.GetMatchingGroup(s.plfilter,tp,LOCATION_DECK|LOCATION_REMOVED,0,nil,tp,szone_chk)) end
    if sp_chk {
        let extra = duel::get_matching_group(d, Some(&|d, c| plfilter(d, c, a.tp, szone_chk)), a.tp, LOCATION_DECK | LOCATION_REMOVED, 0, NO_EX);
        group::add_card(&mut g, &extra);
    }
    // if #g==0 then return end
    if g.is_empty() {
        return;
    }
    // local hint=sp_chk and aux.Stringid(id,2) or HINTMSG_ATOHAND
    let hint = if sp_chk { aux::stringid(ID, 2) } else { HINTMSG_ATOHAND as u64 };
    // Duel.Hint(HINT_SELECTMSG,tp,hint)
    duel::hint(d, HINT_SELECTMSG as u8, a.tp, hint);
    // local sc=g:Select(tp,1,1,nil):GetFirst()
    let sel = group::select(d, &g, a.tp, 1, 1, false, NO_EX).nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &sel) else { return };
    // if sc:IsLocation(LOCATION_GRAVE) then
    if card::is_location(d, sc, LOCATION_GRAVE) {
        // Duel.HintSelection(sc)
        duel::hint_selection(d, sc, true);
        // Duel.SendtoHand(sc,nil,REASON_EFFECT)
        duel::send_to_hand(d, sc, None, REASON_EFFECT, None);
    } else {
        // if sc:IsLocation(LOCATION_REMOVED) then Duel.HintSelection(sc) end
        if card::is_location(d, sc, LOCATION_REMOVED) {
            duel::hint_selection(d, sc, true);
        }
        // local place_location=sc:IsFieldSpell() and LOCATION_FZONE or LOCATION_SZONE
        let place_location = if card::is_field_spell(d, sc) { LOCATION_FZONE } else { LOCATION_SZONE };
        // Duel.MoveToField(sc,tp,tp,place_location,POS_FACEUP,true)
        duel::move_to_field(d, sc, a.tp, a.tp, place_location as u16, POS_FACEUP as u8, true, None);
    }
}
