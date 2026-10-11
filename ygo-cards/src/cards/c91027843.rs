//! Marincess Battle Ocean
use super::prelude::*;

pub const ID: u32 = 91027843;
pub const INFO: ScriptInfo = ScriptInfo { listed_names: &[67712104], listed_series: &[SET_MARINCESS], ..ScriptInfo::DEFAULT };

pub fn initial_effect(d: &mut Duel, c: CardId) {
    // Activate
    // local e1=Effect.CreateEffect(c)
    let e1 = effect::create_effect(d, c);
    // e1:SetType(EFFECT_TYPE_ACTIVATE)
    effect::set_type(d, e1, EFFECT_TYPE_ACTIVATE);
    // e1:SetCode(EVENT_FREE_CHAIN)
    effect::set_code(d, e1, EVENT_FREE_CHAIN);
    // c:RegisterEffect(e1)
    card::register_effect(d, c, e1, false);
    // Increase the ATK of "Marincess" monsters
    // local e2=Effect.CreateEffect(c)
    let e2 = effect::create_effect(d, c);
    // e2:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e2, EFFECT_TYPE_FIELD);
    // e2:SetCode(EFFECT_UPDATE_ATTACK)
    effect::set_code(d, e2, EFFECT_UPDATE_ATTACK);
    // e2:SetRange(LOCATION_FZONE)
    effect::set_range(d, e2, LOCATION_FZONE);
    // e2:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e2, LOCATION_MZONE, 0);
    // e2:SetTarget(aux.TargetBoolFunction(Card.IsSetCard,SET_MARINCESS))
    effect::set_target(d, e2, Some(effect_card_fn(|d, _e, c| card::is_set_card(d, c, &[SET_MARINCESS as u16]))));
    // e2:SetValue(s.atkval)
    effect::set_value_fn(d, e2, effect_card_value(atkval));
    // c:RegisterEffect(e2)
    card::register_effect(d, c, e2, false);
    // Monsters in the Extra Monster zone becomes unaffected
    // local e3=Effect.CreateEffect(c)
    let e3 = effect::create_effect(d, c);
    // e3:SetDescription(aux.Stringid(id,0))
    effect::set_description(d, e3, aux::stringid(ID, 0));
    // e3:SetType(EFFECT_TYPE_FIELD)
    effect::set_type(d, e3, EFFECT_TYPE_FIELD);
    // e3:SetCode(EFFECT_IMMUNE_EFFECT)
    effect::set_code(d, e3, EFFECT_IMMUNE_EFFECT);
    // e3:SetRange(LOCATION_FZONE)
    effect::set_range(d, e3, LOCATION_FZONE);
    // e3:SetTargetRange(LOCATION_MZONE,0)
    effect::set_target_range(d, e3, LOCATION_MZONE, 0);
    // e3:SetTarget(s.etarget)
    effect::set_target(d, e3, Some(effect_card_fn(etarget)));
    // e3:SetValue(s.efilter)
    effect::set_value_fn(d, e3, lua_fn(efilter));
    // c:RegisterEffect(e3)
    card::register_effect(d, c, e3, false);
    // Equip up to 3 "Marincess" monsters from the GY to a monster Link Summoned
    // local e4=Effect.CreateEffect(c)
    let e4 = effect::create_effect(d, c);
    // e4:SetDescription(aux.Stringid(id,1))
    effect::set_description(d, e4, aux::stringid(ID, 1));
    // e4:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_TRIGGER_O)
    effect::set_type(d, e4, EFFECT_TYPE_FIELD + EFFECT_TYPE_TRIGGER_O);
    // e4:SetCode(EVENT_SPSUMMON_SUCCESS)
    effect::set_code(d, e4, EVENT_SPSUMMON_SUCCESS);
    // e4:SetRange(LOCATION_FZONE)
    effect::set_range(d, e4, LOCATION_FZONE);
    // e4:SetCondition(s.eqcon)
    effect::set_condition(d, e4, Some(condition(eqcon)));
    // e4:SetTarget(s.eqtg)
    effect::set_target(d, e4, Some(cost_target(eqtg)));
    // e4:SetOperation(s.eqop)
    effect::set_operation(d, e4, Some(operation(eqop)));
    // c:RegisterEffect(e4)
    card::register_effect(d, c, e4, false);
    // Material check for "Marincess Crystal Heart"
    // aux.GlobalCheck(s,function()
    aux::global_check(d, ID, |d| {
        // local e1=Effect.GlobalEffect()
        let e1 = effect::global_effect(d);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_MATERIAL_CHECK)
        effect::set_code(d, e1, EFFECT_MATERIAL_CHECK);
        // e1:SetValue(s.matchk)
        effect::set_value_fn(d, e1, lua_fn(matchk));
        // local ge1=Effect.GlobalEffect()
        let ge1 = effect::global_effect(d);
        // ge1:SetType(EFFECT_TYPE_FIELD+EFFECT_TYPE_GRANT)
        effect::set_type(d, ge1, EFFECT_TYPE_FIELD + EFFECT_TYPE_GRANT);
        // ge1:SetLabelObject(e1)
        effect::set_label_object(d, ge1, Value::Effect(e1));
        // ge1:SetTargetRange(LOCATION_ALL,LOCATION_ALL)
        effect::set_target_range(d, ge1, LOCATION_ALL, LOCATION_ALL);
        // Duel.RegisterEffect(ge1,0)
        duel::register_effect(d, ge1, 0);
    });
}
// function s.matchk(e,c)
fn matchk(d: &mut Duel, a: Args) {
    // if not (c and c:IsType(TYPE_LINK)) then return end
    let Some(c) = a.card(1) else { return };
    if !card::is_type(d, c, TYPE_LINK) {
        return;
    }
    // local g=c:GetMaterial()
    let g = card::get_material(d, c);
    // if g:IsExists(Card.IsSummonCode,1,nil,c,SUMMON_TYPE_LINK,e:GetHandlerPlayer(),67712104) then
    let hp = effect::get_handler_player(d, a.e(0));
    if group::is_exists(d, &g, &|d, x| card::is_summon_code(d, x, Some(c), SUMMON_TYPE_LINK as u64, hp, &[67712104]), 1, NO_EX) {
        // c:RegisterFlagEffect(id,RESET_EVENT|RESETS_STANDARD&~(RESET_TOFIELD|RESET_LEAVE|RESET_TEMP_REMOVE),EFFECT_FLAG_CLIENT_HINT,1,0,aux.Stringid(id,0))
        card::register_flag_effect(
            d,
            c,
            ID,
            RESET_EVENT | (RESETS_STANDARD & !(RESET_TOFIELD | RESET_LEAVE | RESET_TEMP_REMOVE)),
            EFFECT_FLAG_CLIENT_HINT,
            1,
            0,
            aux::stringid(ID, 0),
        );
    }
}
// function s.atkval(e,c)
fn atkval(d: &mut Duel, _e: EffectId, c: CardId) -> i64 {
    // return 200+c:GetEquipGroup():FilterCount(Card.IsSetCard,nil,SET_MARINCESS)*600
    let eq = card::get_equip_group(d, c);
    200 + group::filter_count(d, &eq, &|d, x| card::is_set_card(d, x, &[SET_MARINCESS as u16]), NO_EX) as i64 * 600
}
// function s.etarget(e,c)
fn etarget(d: &mut Duel, _e: EffectId, c: CardId) -> bool {
    // return c:IsFaceup() and c:IsInExtraMZone() and c:HasFlagEffect(id) and c:IsLinkSummoned()
    card::is_faceup(d, c)
        && card::is_in_extra_m_zone(d, c, None)
        && card::has_flag_effect(d, c, ID, None)
        && card::is_link_summoned(d, c)
}
// function s.efilter(e,re)
fn efilter(d: &mut Duel, a: Args) -> bool {
    // return e:GetOwnerPlayer()~=re:GetOwnerPlayer()
    effect::get_owner_player(d, a.e(0)) != effect::get_owner_player(d, a.e(1))
}
// function s.eqcfilter(c,tp)
fn eqcfilter(d: &mut Duel, c: CardId, tp: u8) -> bool {
    // return c:IsSetCard(SET_MARINCESS) and c:IsLinkMonster() and c:IsLinkSummoned() and c:GetSequence()>=5 and c:IsSummonPlayer(tp)
    card::is_set_card(d, c, &[SET_MARINCESS as u16])
        && card::is_link_monster(d, c)
        && card::is_link_summoned(d, c)
        && card::get_sequence(d, c) >= 5
        && card::is_summon_player(d, c, tp)
}
// function s.eqfilter(c)
fn eqfilter(d: &mut Duel, c: CardId) -> bool {
    // return c:IsSetCard(SET_MARINCESS) and c:IsLinkMonster() and not c:IsForbidden()
    card::is_set_card(d, c, &[SET_MARINCESS as u16]) && card::is_link_monster(d, c) && !card::is_forbidden(d, c)
}
// function s.eqcon(e,tp,eg,ep,ev,re,r,rp)
fn eqcon(d: &mut Duel, a: &EffArgs) -> bool {
    // return eg:IsExists(s.eqcfilter,1,nil,tp)
    let tp = a.tp;
    group::is_exists(d, &a.eg(), &|d, c| eqcfilter(d, c, tp), 1, NO_EX)
}
// function s.eqtg(e,tp,eg,ep,ev,re,r,rp,chk)
fn eqtg(d: &mut Duel, a: &EffArgs) -> bool {
    // if chk==0 then return Duel.GetLocationCount(tp,LOCATION_SZONE)>0
    //     and Duel.IsExistingMatchingCard(s.eqfilter,tp,LOCATION_GRAVE,0,1,nil) end
    if a.chk == Some(0) {
        return duel::get_location_count(d, a.tp, LOCATION_SZONE as u8, None, None, None).0 > 0
            && duel::is_existing_matching_card(d, Some(&eqfilter), a.tp, LOCATION_GRAVE, 0, 1, NO_EX);
    }
    // Duel.SetOperationInfo(0,CATEGORY_LEAVE_GRAVE,nil,1,0,0)
    duel::set_operation_info(d, 0, CATEGORY_LEAVE_GRAVE, None, 1, 0, 0);
    true
}
// function s.eqop(e,tp,eg,ep,ev,re,r,rp)
fn eqop(d: &mut Duel, a: &EffArgs) {
    let tp = a.tp;
    // local ft=Duel.GetLocationCount(tp,LOCATION_SZONE)
    let ft = duel::get_location_count(d, tp, LOCATION_SZONE as u8, None, None, None).0;
    // local tc=eg:Filter(s.eqcfilter,nil,tp):GetFirst()
    let eg = a.eg();
    let filtered = group::filter(d, &eg, &|d, c| eqcfilter(d, c, tp), NO_EX);
    let tc = group::get_first(d, &filtered);
    // if ft<1 or not tc or not tc:IsFaceup() then return end
    let Some(tc) = tc else { return };
    if ft < 1 || !card::is_faceup(d, tc) {
        return;
    }
    // local g=aux.SelectUnselectGroup(Duel.GetMatchingGroup(aux.NecroValleyFilter(s.eqfilter),tp,LOCATION_GRAVE,0,nil),e,tp,1,math.min(ft,3),aux.dncheck,1,tp,HINTMSG_EQUIP)
    let mg = duel::get_matching_group(d, Some(&aux::necro_valley_filter(eqfilter)), tp, LOCATION_GRAVE, 0, NO_EX);
    let dn = aux::dncheck();
    let maxc = std::cmp::min(ft, 3) as usize;
    let g = aux::select_unselect_group(d, &mg, Some(a.e), tp, Some(1), Some(maxc), Some(&dn), 1, tp, Some(HINTMSG_EQUIP as u64), None, None, false)
        .unwrap_or_default();
    // for eqc in g:Iter() do
    for eqc in group::iter(d, &g) {
        // Duel.Equip(tp,eqc,tc,true,true)
        duel::equip(d, tp, eqc, tc, true, true);
        // local e1=Effect.CreateEffect(e:GetHandler())
        let h = effect::get_handler(d, a.e).nil();
        let e1 = effect::create_effect(d, h);
        // e1:SetType(EFFECT_TYPE_SINGLE)
        effect::set_type(d, e1, EFFECT_TYPE_SINGLE);
        // e1:SetCode(EFFECT_EQUIP_LIMIT)
        effect::set_code(d, e1, EFFECT_EQUIP_LIMIT);
        // e1:SetProperty(EFFECT_FLAG_CANNOT_DISABLE+EFFECT_FLAG_IGNORE_IMMUNE)
        effect::set_property(d, e1, EFFECT_FLAG_CANNOT_DISABLE + EFFECT_FLAG_IGNORE_IMMUNE, 0);
        // e1:SetReset(RESET_EVENT|RESETS_STANDARD)
        effect::set_reset(d, e1, RESET_EVENT | RESETS_STANDARD, 1);
        // e1:SetValue(s.eqlimit)
        effect::set_value_fn(d, e1, effect_card_fn(eqlimit));
        // e1:SetLabelObject(tc)
        effect::set_label_object(d, e1, Value::Card(tc));
        // eqc:RegisterEffect(e1,true)
        card::register_effect(d, eqc, e1, true);
    }
    // Duel.EquipComplete()
    duel::equip_complete(d);
}
// function s.eqlimit(e,c)
fn eqlimit(d: &mut Duel, e: EffectId, c: CardId) -> bool {
    // return c==e:GetLabelObject()
    effect::get_label_object(d, e).as_card() == Some(c)
}
