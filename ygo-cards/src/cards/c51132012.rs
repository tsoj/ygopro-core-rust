//! Grand Knight of the Red Lotus
//! scripted by pyrQ
use super::prelude::*;

pub const ID: u32 = 51132012;

pub const INFO: ScriptInfo = ScriptInfo {
    // s.listed_names={id}
    listed_names: &[ID],
    ..ScriptInfo::DEFAULT
};

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // c:EnableReviveLimit()
    card::enable_revive_limit(d, c);
    // c:AddMustBeSpecialSummoned()
    let _ = card::add_must_be_special_summoned(d, c);
    // Must be Special Summoned (from your hand or GY) by shuffling up to 3 non-Effect Monsters from your GY into the Deck/Extra Deck
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e1, EFFECT_TYPE_FIELD);
    // e1:SetProperty(EFFECT_FLAG_UNCOPYABLE+EFFECT_FLAG_CANNOT_DISABLE)
    effect::set_property(d, e1, EFFECT_FLAG_UNCOPYABLE + EFFECT_FLAG_CANNOT_DISABLE, 0);
    // e1:SetCode(EFFECT_SPSUMMON_PROC)
    effect::set_code(d, e1, EFFECT_SPSUMMON_PROC);
    // e1:SetRange(LOCATION_HAND|LOCATION_GRAVE)
    effect::set_range(d, e1, LOCATION_HAND | LOCATION_GRAVE);
    // e1:SetCountLimit(1,id,EFFECT_COUNT_CODE_OATH)
    effect::set_count_limit(d, e1, 1, ID, 0, EFFECT_COUNT_CODE_OATH as u8);
    // e1:SetCondition(s.spcon)
    effect::set_condition(d, e1, Some(lua_fn(spcon)));
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(lua_fn(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(lua_fn(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);

    // Destroy 1 monster on the field and Special Summon 1 non-Effect Monster from your GY
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2, aux::stringid(ID, 1));
    // e2:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e2, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e2:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e2, EFFECT_TYPE_IGNITION);
    // e2:SetProperty(EFFECT_FLAG_CARD_TARGET)
    effect::set_property(d, e2, EFFECT_FLAG_CARD_TARGET, 0);
    // e2:SetRange(LOCATION_MZONE)
    effect::set_range(d, e2, LOCATION_MZONE);
    // e2:SetCountLimit(1)
    effect::set_count_limit(d, e2, 1, 0, 0, 0);
    // e2:SetTarget(s.destg)
    effect::set_target(d, e2, Some(cost_target(destg)));
    // e2:SetOperation(s.desop)
    effect::set_operation(d, e2, Some(operation(desop)));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
}
// function s.spcon(e,c)
fn spcon(d: &mut Duel, a: Args) -> bool {
    // if c==nil then return true end
    if a.card(1).is_none() {
        return true;
    }
    // local tp=e:GetHandlerPlayer()
    let tp = effect::get_handler_player(d, a.e(0));
    // return Duel.GetLocationCount(tp,LOCATION_MZONE)>0
    //   and Duel.IsExistingMatchingCard(aux.AND(Card.IsNonEffectMonster,Card.IsAbleToDeckOrExtraAsCost),tp,LOCATION_GRAVE,0,1,e:GetHandler())
    duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0 && {
        let h = effect::get_handler(d, a.e(0)).nil();
        duel::is_existing_matching_card(
            d,
            Some(&|d, c| card::is_non_effect_monster(d, c) && card::is_able_to_deck_or_extra_as_cost(d, c)),
            tp,
            LOCATION_GRAVE,
            0,
            1,
            h.into(),
        )
    }
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk,c)
fn sptg(d: &mut Duel, a: Args) -> bool {
    let e = a.e(0);
    let tp = a.p(1);
    let c = a.card(9);
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_TODECK)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_TODECK as u64);
    // local g=Duel.SelectMatchingCard(tp,aux.AND(Card.IsNonEffectMonster,Card.IsAbleToDeckOrExtraAsCost),tp,LOCATION_GRAVE,0,1,3,true,c)
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, cc| card::is_non_effect_monster(d, cc) && card::is_able_to_deck_or_extra_as_cost(d, cc)),
        tp,
        LOCATION_GRAVE,
        0,
        1,
        3,
        true,
        match c {
            Some(c) => c.into(),
            None => NO_EX,
        },
    );
    // if g and #g>0 then
    if let Some(g) = g {
        if g.len() > 0 {
            // g:KeepAlive()
            // e:SetLabelObject(g)
            effect::set_label_object(d, e, Value::Group(g));
            // return true
            return true;
        }
    }
    // return false
    false
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp,c)
fn spop(d: &mut Duel, a: Args) {
    let e = a.e(0);
    let c = a.c(8);
    // local g=e:GetLabelObject()
    // if not g then return end
    let Some(g) = effect::get_label_object(d, e).as_group().cloned() else {
        return;
    };
    // Duel.HintSelection(g)
    duel::hint_selection(d, g.clone(), true);
    // Duel.SendtoDeck(g,nil,SEQ_DECKSHUFFLE,REASON_COST)
    duel::send_to_deck(d, g.clone(), None, SEQ_DECKSHUFFLE as i32, REASON_COST, None);
    // local atk=g:FilterCount(Card.IsType,nil,TYPE_NORMAL)*800
    let atk = group::filter_count(d, &g, &|d, cc| card::is_type(d, cc, TYPE_NORMAL), NO_EX) as i64 * 800;
    // g:DeleteGroup()
    // if atk==0 then return end
    if atk == 0 {
        return;
    }
    // It gains 800 ATK for each Normal Monster shuffled into the Deck for its Summon
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
    // e1:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e1, EFFECT_UPDATE_ATTACK);
    // e1:SetValue(atk)
    effect::set_value(d, e1, atk);
    // e1:SetReset(RESET_EVENT|RESETS_STANDARD_DISABLE&~RESET_TOFIELD)
    effect::set_reset(d, e1, RESET_EVENT | (RESETS_STANDARD_DISABLE & !RESET_TOFIELD), 1);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
}
// function s.desfilter(c,tp)
fn desfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return Duel.GetMZoneCount(tp,c)>0
    duel::get_m_zone_count(d, tp, c.into(), None, None, None).0 > 0
}
// function s.spfilter(c,e,tp)
fn spfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8) -> bool {
    // return c:IsNonEffectMonster() and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
    card::is_non_effect_monster(d, c) && card::is_can_be_special_summoned(d, c, e, 0, tp, false, false, None, None, None)
}
// function s.destg(e,tp,eg,ep,ev,re,r,rp,chk,chkc)
fn destg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chkc then return false end
    if a.chkc().is_some() {
        return false;
    }
    // if chk==0 then return Duel.IsExistingTarget(s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,nil,tp)
    //   and Duel.IsExistingTarget(s.spfilter,tp,LOCATION_GRAVE,0,1,nil,e,tp) end
    if a.chk == Some(0) {
        return duel::is_existing_target(d, Some(&|d, c| desfilter(d, c, a.tp)), a.tp, LOCATION_MZONE, LOCATION_MZONE, 1, NO_EX)
            && duel::is_existing_target(d, Some(&|d, c| spfilter(d, c, a.e, a.tp)), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    let tp = a.tp;
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local desg=Duel.SelectTarget(tp,s.desfilter,tp,LOCATION_MZONE,LOCATION_MZONE,1,1,nil,tp)
    let desg = duel::select_target(d, tp, Some(&|d, c| desfilter(d, c, tp)), tp, LOCATION_MZONE, LOCATION_MZONE, 1, 1, false, NO_EX).nil();
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_SPSUMMON)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_SPSUMMON as u64);
    // local spg=Duel.SelectTarget(tp,s.spfilter,tp,LOCATION_GRAVE,0,1,1,nil,e,tp)
    let spg = duel::select_target(d, tp, Some(&|d, c| spfilter(d, c, a.e, tp)), tp, LOCATION_GRAVE, 0, 1, 1, false, NO_EX).nil();
    // Duel.SetOperationInfo(0,CATEGORY_DESTROY,desg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(desg), 1, tp, 0);
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,spg,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(spg), 1, tp, 0);
    true
}
// function s.desop(e,tp,eg,ep,ev,re,r,rp)
fn desop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local tg=Duel.GetTargetCards(e)
    let tg = duel::get_target_cards(d, a.e).nil();
    // if #tg==0 then return end
    if tg.len() == 0 {
        return;
    }
    // local field_tc=tg:Filter(Card.IsLocation,nil,LOCATION_MZONE):GetFirst()
    let filtered = group::filter(d, &tg, &|d, c| card::is_location(d, c, LOCATION_MZONE), NO_EX);
    let field_tc = group::get_first(d, &filtered);
    // if field_tc and Duel.Destroy(field_tc,REASON_EFFECT)>0 then
    if let Some(field_tc) = field_tc {
        if duel::destroy(d, CardSet::from_card(field_tc), REASON_EFFECT, None, None) > 0 {
            // local gy_tc=(tg-field_tc):GetFirst()
            let mut rest = tg.clone();
            group::remove_card(&mut rest, &CardSet::from_card(field_tc));
            let gy_tc = group::get_first(d, &rest);
            // if gy_tc then Duel.SpecialSummon(gy_tc,0,tp,tp,false,false,POS_FACEUP) end
            if let Some(gy_tc) = gy_tc {
                duel::special_summon(d, CardSet::from_card(gy_tc), 0, tp, tp, false, false, POS_FACEUP as u8, None);
            }
        }
    }
}
