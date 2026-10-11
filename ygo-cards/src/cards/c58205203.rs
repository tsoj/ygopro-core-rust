//! Mariamne Paradox, the True Dracophoenix Knight
//! Scripted by The Razgriz
use super::prelude::*;

pub const ID: u32 = 58205203;

pub const INFO: ScriptInfo = ScriptInfo { listed_series: &[SET_TRUE_DRACO_KING], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // To Tribute Summon this card face-up, you can Tribute a Continuous Spell/Trap you control, instead of a monster
    // local e0=Effect.CreateEffect(c)
    let e0 = effect::create_effect(d, c);
    // e0:SetType(EFFECT_TYPE_SINGLE)
    effect::set_type(d, e0, EFFECT_TYPE_SINGLE);
    // e0:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_UNCOPYABLE)
    effect::set_property(d, e0, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_UNCOPYABLE, 0);
    // e0:SetCode(EFFECT_ADD_EXTRA_TRIBUTE)
    effect::set_code(d, e0, EFFECT_ADD_EXTRA_TRIBUTE);
    // e0:SetTargetRange(LOCATION_SZONE,0)
    effect::set_target_range(d, e0, LOCATION_SZONE, 0);
    // e0:SetTarget(aux.TargetBoolFunction(Card.IsContinuousSpellTrap))
    effect::set_target(d, e0, Some(effect_card_fn(|d, _e, c| card::is_continuous_spell_trap(d, c))));
    // e0:SetValue(POS_FACEUP)
    effect::set_value(d, e0, POS_FACEUP as i64);
    // c:RegisterEffect(e0)
    card::register_effect(d, c, e0, false);
    // Destroy 1 other "True Draco" or "True King" card in your hand or face-up field, and if you do, Special Summon this card to either field
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e1, aux::stringid(ID, 0));
    // e1:SetCategory(CATEGORY_DESTROY+CATEGORY_SPECIAL_SUMMON)
    effect::set_category(d, e1, CATEGORY_DESTROY + CATEGORY_SPECIAL_SUMMON);
    // e1:SetType(EFFECT_TYPE_IGNITION)
    effect::set_type(d, e1, EFFECT_TYPE_IGNITION);
    // e1:SetRange(LOCATION_HAND)
    effect::set_range(d, e1, LOCATION_HAND);
    // e1:SetCountLimit(1,id)
    effect::set_count_limit(d, e1, 1, ID, 0, 0);
    // e1:SetTarget(s.sptg)
    effect::set_target(d, e1, Some(cost_target(sptg)));
    // e1:SetOperation(s.spop)
    effect::set_operation(d, e1, Some(operation(spop)));
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Banish the top 4 cards of your Deck
    // local e2a=Effect.CreateEffect(c)
    let e2a = effect::create_effect(d, c);
    // e2a:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e2a, aux::stringid(ID, 1));
    // e2a:SetCategory(CATEGORY_REMOVE)
    effect::set_category(d, e2a, CATEGORY_REMOVE);
    // e2a:SetType(EFFECT_TYPE_SINGLE+EFFECT_TYPE_TRIGGER_F)
    effect::set_type(d, e2a, EFFECT_TYPE_SINGLE + EFFECT_TYPE_TRIGGER_F);
    // e2a:SetCode(EVENT_SUMMON_SUCCESS)
    effect::set_code(d, e2a, EVENT_SUMMON_SUCCESS);
    // e2a:SetCountLimit(1,{id,1})
    effect::set_count_limit(d, e2a, 1, ID, 1, 0);
    // e2a:SetCondition(function(e) return e:GetHandler():IsSummonLocation(LOCATION_HAND) end)
    effect::set_condition(d, e2a, Some(condition(cond2a)));
    // e2a:SetTarget(s.rmtg)
    effect::set_target(d, e2a, Some(cost_target(rmtg)));
    // e2a:SetOperation(s.rmop)
    effect::set_operation(d, e2a, Some(operation(rmop)));
    // c:RegisterEffect(e2a)
    card::register_effect(d, c, e2a, false);
    // local e2b=e2a:Clone()
    let e2b = effect::clone(d, e2a);
    // e2b:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e2b, EVENT_SPSUMMON_SUCCESS);
    // c:RegisterEffect(e2b)
    card::register_effect(d, c, e2b, false);
}
// function s.desfilter(c,e,tp,hc,resolution_chk)
fn desfilter(d: &mut Duel, c: CardId, e: EffectId, tp: u8, hc: CardId, resolution_chk: bool) -> bool {
    // return c:IsSetCard(SET_TRUE_DRACO_KING) and (c:IsLocation(LOCATION_HAND) or c:IsFaceup())
    //     and (resolution_chk or s.spcheck(c,e,tp,hc))
    card::is_set_card(d, c, &[SET_TRUE_DRACO_KING as u16])
        && (card::is_location(d, c, LOCATION_HAND) || card::is_faceup(d, c))
        && (resolution_chk || spcheck(d, c, e, tp, hc))
}
// function s.spcheck(c,e,tp,hc)
fn spcheck(d: &mut Duel, c: CardId, e: EffectId, tp: u8, hc: CardId) -> bool {
    // local ctrl=c:GetControler()
    let ctrl = card::get_controler(d, c);
    // return (Duel.GetMZoneCount(ctrl,c,tp)>0 and hc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,ctrl))
    //     or (Duel.GetMZoneCount(1-ctrl,nil,tp)>0 and hc:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-ctrl))
    (duel::get_m_zone_count(d, ctrl, c.into(), Some(tp), None, None).0 > 0
        && card::is_can_be_special_summoned(d, hc, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(ctrl), None))
        || (duel::get_m_zone_count(d, 1 - ctrl, NO_EX, Some(tp), None, None).0 > 0
            && card::is_can_be_special_summoned(d, hc, e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - ctrl), None))
}
// local oppo_location=Duel.IsPlayerAffectedByEffect(tp,88581108) and LOCATION_MZONE or 0
fn oppo_location(d: &mut Duel, tp: u8) -> u32 {
    if duel::is_player_affected_by_effect(d, tp, 88581108).is_some() {
        LOCATION_MZONE
    } else {
        0
    }
}
// function s.sptg(e,tp,eg,ep,ev,re,r,rp,chk)
fn sptg(d: &mut Duel, a: &EffArgs) -> bool {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local oppo_location=...
    let oppo = oppo_location(d, tp);
    // local g=Duel.GetMatchingGroup(s.desfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,oppo_location,c,e,tp,c,false)
    let g = duel::get_matching_group(d, Some(&|d, x| desfilter(d, x, a.e, tp, c, false)), tp, LOCATION_HAND | LOCATION_ONFIELD, oppo, c.into());
    // if chk==0 then return #g>0 end
    if a.chk == Some(0) {
        return !g.is_empty();
    }
    // if not g:IsExists(Card.IsOnField,1,nil) or ((Duel.GetMZoneCount(tp)>0 or Duel.GetMZoneCount(1-tp)>0)
    //     and (Duel.IsExistingMatchingCard(aux.NOT(Card.IsPublic),tp,LOCATION_HAND,0,1,c)
    //     or g:IsExists(aux.FaceupFilter(Card.IsLocation,LOCATION_HAND),1,nil))) then
    // (lazy like the Lua `or`/`and`: the later checks run only when needed)
    let no_field = !group::is_exists(d, &g, &|d, x| card::is_on_field(d, x), 1, NO_EX);
    let cond = no_field
        || ((duel::get_m_zone_count(d, tp, NO_EX, None, None, None).0 > 0 || duel::get_m_zone_count(d, 1 - tp, NO_EX, None, None, None).0 > 0)
            && (duel::is_existing_matching_card(d, Some(&|d, x| !card::is_public(d, x)), tp, LOCATION_HAND, 0, 1, c.into())
                || group::is_exists(d, &g, &|d, x| card::is_faceup(d, x) && card::is_location(d, x, LOCATION_HAND), 1, NO_EX)));
    if cond {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,nil,1,tp,LOCATION_HAND|LOCATION_ONFIELD)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, None, 1, tp, (LOCATION_HAND | LOCATION_ONFIELD) as i32);
    } else {
        // Duel.SetOperationInfo(0,CATEGORY_DESTROY,g,1,tp,0)
        duel::set_operation_info(d, 0, CATEGORY_DESTROY, Some(g), 1, tp, 0);
    }
    // Duel.SetOperationInfo(0,CATEGORY_SPECIAL_SUMMON,c,1,tp,0)
    duel::set_operation_info(d, 0, CATEGORY_SPECIAL_SUMMON, Some(CardSet::from_card(c)), 1, tp, 0);
    true
}
// function s.spop(e,tp,eg,ep,ev,re,r,rp)
fn spop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local c=e:GetHandler()
    let c = effect::get_handler(d, a.e).nil();
    // local relation_chk=c:IsRelateToEffect(e)
    let relation_chk = card::is_relate_to_effect(d, c, a.e);
    // local exc=relation_chk and c or nil
    let exc: Exclude = if relation_chk { c.into() } else { NO_EX };
    // local oppo_location=...
    let oppo = oppo_location(d, tp);
    // local resolution_chk=not (relation_chk and Duel.IsExistingMatchingCard(s.desfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,oppo_location,1,exc,e,tp,c,false))
    let resolution_chk = !(relation_chk
        && duel::is_existing_matching_card(d, Some(&|d, x| desfilter(d, x, a.e, tp, c, false)), tp, LOCATION_HAND | LOCATION_ONFIELD, oppo, 1, exc));
    // Duel.Hint(HINT_SELECTMSG,tp,HINTMSG_DESTROY)
    duel::hint(d, HINT_SELECTMSG as u8, tp, HINTMSG_DESTROY as u64);
    // local sc=Duel.SelectMatchingCard(tp,s.desfilter,tp,LOCATION_HAND|LOCATION_ONFIELD,oppo_location,1,1,exc,e,tp,c,resolution_chk):GetFirst()
    let g = duel::select_matching_card(
        d,
        tp,
        Some(&|d, x| desfilter(d, x, a.e, tp, c, resolution_chk)),
        tp,
        LOCATION_HAND | LOCATION_ONFIELD,
        oppo,
        1,
        1,
        false,
        exc,
    )
    .nil();
    // if not sc then return end
    let Some(sc) = group::get_first(d, &g) else {
        return;
    };
    // if sc:IsOnField() then Duel.HintSelection(sc) end
    if card::is_on_field(d, sc) {
        duel::hint_selection(d, sc, true);
    }
    // if Duel.Destroy(sc,REASON_EFFECT)>0 and relation_chk then
    if duel::destroy(d, sc, REASON_EFFECT, None, None) > 0 && relation_chk {
        // local b1=Duel.GetLocationCount(tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false)
        let b1 = duel::get_location_count(d, tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, None, None, None);
        // local b2=Duel.GetLocationCount(1-tp,LOCATION_MZONE)>0 and c:IsCanBeSpecialSummoned(e,0,tp,false,false,POS_FACEUP,1-tp)
        let b2 = duel::get_location_count(d, 1 - tp, LOCATION_MZONE as u8, None, None, None).0 > 0
            && card::is_can_be_special_summoned(d, c, a.e, 0, tp, false, false, Some(POS_FACEUP as u8), Some(1 - tp), None);
        // if not (b1 or b2) then return end
        if !(b1 || b2) {
            return;
        }
        // local op=Duel.SelectEffect(tp,{b1,aux.Stringid(id,2)},{b2,aux.Stringid(id,3)})
        let op = duel::select_effect(d, tp, &[(b1, aux::stringid(ID, 2)), (b2, aux::stringid(ID, 3))]);
        // local target_player=op==1 and tp or 1-tp
        let target_player = if op == Some(1) { tp } else { 1 - tp };
        // Duel.SpecialSummon(c,0,tp,target_player,false,false,POS_FACEUP)
        duel::special_summon(d, c, 0, tp, target_player, false, false, POS_FACEUP as u8, None);
    }
}
// function s.rmtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn rmtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return true end
    if a.chk == Some(0) {
        return true;
    }
    // local g=Duel.GetDecktopGroup(tp,4)
    let g = duel::get_decktop_group(d, a.tp, 4);
    // Duel.SetOperationInfo(0,CATEGORY_REMOVE,g,#g,tp,0)
    let n = g.len() as u8;
    duel::set_operation_info(d, 0, CATEGORY_REMOVE, Some(g), n, a.tp, 0);
    true
}
// function s.rmop(e,tp,eg,ep,ev,re,r,rp)
fn rmop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local g=Duel.GetDecktopGroup(tp,4)
    let g = duel::get_decktop_group(d, tp, 4);
    // if #g>0 then
    if !g.is_empty() {
        // Duel.DisableShuffleCheck()
        duel::disable_shuffle_check(d, true);
        // Duel.Remove(g,POS_FACEUP,REASON_EFFECT)
        duel::remove(d, g, POS_FACEUP as u8, REASON_EFFECT, None, None);
    }
}
// function(e) return e:GetHandler():IsSummonLocation(LOCATION_HAND) end
fn cond2a(d: &mut Duel, a: &EffArgs) -> bool {
    let c = effect::get_handler(d, a.e).nil();
    card::is_summon_location(d, c, LOCATION_HAND)
}
